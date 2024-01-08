use std::error::Error;
use std::slice;
use libc::c_void;

fn main() {}

#[repr(C)]
pub struct CompressionResult {
    success: bool,
    error_code: u8,
    pointer: *mut u8,
    len: usize,
    capacity: usize,
}

#[no_mangle]
pub extern "C" fn drop_vector_struct(data_ptr: *mut c_void) {
    let data: Box<CompressionResult> = unsafe {
        Box::from_raw(data_ptr as *mut CompressionResult)
    };

    let _ = unsafe {
        Vec::from_raw_parts(
            data.pointer,
            data.len,
            data.capacity,
        )
    };
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern fn w_compress(input: *const u8, input_size: usize, quality: i32, keep_metadata: i32) -> *mut c_void {
    let uncompressed_buffer = slice::from_raw_parts(input, input_size);
    let keep_metadata = keep_metadata != 0;
    let mut compressed_buffer = perform_compression(uncompressed_buffer, quality as u32, keep_metadata);

    let data = CompressionResult {
        pointer: compressed_buffer.as_mut_ptr(),
        len: compressed_buffer.len(),
        capacity: compressed_buffer.capacity(),
    };

    let data_ptr: *mut c_void = Box::into_raw(Box::new(data)) as *mut c_void;
    std::mem::forget(compressed_buffer);

    data_ptr
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern fn w_compress_to_size(input: *const u8, input_size: usize, max_size: i32, keep_metadata: i32) -> *mut c_void {
    let uncompressed_buffer = slice::from_raw_parts(input, input_size);
    let keep_metadata = keep_metadata != 0;
    let mut compressed_buffer = perform_compression_to_size(uncompressed_buffer, max_size as u32, keep_metadata);

    let data = CompressionResult {
        success: true,

        pointer: compressed_buffer.as_mut_ptr(),
        len: compressed_buffer.len(),
        capacity: compressed_buffer.capacity(),
    };

    let data_ptr: *mut c_void = Box::into_raw(Box::new(data)) as *mut c_void;
    std::mem::forget(compressed_buffer);

    data_ptr
}

fn perform_compression_to_size(file: &[u8], max_size: u32, keep_metadata: bool) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut parameters = caesium::initialize_parameters();
    parameters.keep_metadata = keep_metadata;
    let in_file = file.to_vec();
    caesium::compress_to_size_in_memory(in_file, &mut parameters, max_size as usize)
}

fn perform_compression(file: &[u8], quality: u32, keep_metadata: bool) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut parameters = caesium::initialize_parameters();
    parameters.keep_metadata = keep_metadata;
    let quality = quality.clamp(0, 100);
    if quality == 0 {
        parameters.optimize = true;
    } else {
        parameters.jpeg.quality = quality;
        parameters.png.quality = quality;
        parameters.webp.quality = quality;
    }
    let in_file = file.to_vec();
    caesium::compress_in_memory(in_file, &mut parameters)
}