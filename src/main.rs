use std::ptr::null_mut;
use std::slice;
use caesium::CaesiumError;
use libc::c_void;

fn main() {}

#[repr(C)]
pub struct CompressionResult {
    status: u32,
    error_code: u32,
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

    handle_result(perform_compression(uncompressed_buffer, quality as u32, keep_metadata))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern fn w_compress_to_size(input: *const u8, input_size: usize, max_size: i32, keep_metadata: i32) -> *mut c_void {
    let uncompressed_buffer = slice::from_raw_parts(input, input_size);
    let keep_metadata = keep_metadata != 0;
    handle_result(perform_compression_to_size(uncompressed_buffer, max_size as u32, keep_metadata))
}

fn handle_result(result: Result<Vec<u8>, CaesiumError>) -> *mut c_void {
    let mut status:u8 = 0;
    let mut error_code: u32 = 0;
    let mut pointer: *mut u8 = null_mut();
    let mut len = 0;
    let mut capacity = 0;

    match result {
        Ok(mut buffer) => {
            status = 1;
            pointer = buffer.as_mut_ptr();
            len = buffer.len();
            capacity = buffer.capacity();

            std::mem::forget(buffer);
        }
        Err(e) => {
            error_code = e.code;
            println!("{}", e.message);
        }
    };

    let data = CompressionResult {
        status: status.into(),
        error_code,
        pointer,
        len,
        capacity,
    };

    Box::into_raw(Box::new(data)) as *mut c_void
}

fn perform_compression_to_size(file: &[u8], max_size: u32, keep_metadata: bool) -> Result<Vec<u8>, CaesiumError> {
    let mut parameters = caesium::initialize_parameters();
    parameters.keep_metadata = keep_metadata;
    let in_file = file.to_vec();
    caesium::compress_to_size_in_memory(in_file, &mut parameters, max_size as usize, true)
}

fn perform_compression(file: &[u8], quality: u32, keep_metadata: bool) -> Result<Vec<u8>, CaesiumError> {
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