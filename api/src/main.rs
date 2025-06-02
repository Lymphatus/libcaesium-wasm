use std::ptr::null_mut;
use std::slice;

use caesium::error::CaesiumError;
use caesium::parameters::{CSParameters, ChromaSubsampling, TiffDeflateLevel};
use caesium::parameters::TiffCompression::{Deflate, Lzw, Packbits, Uncompressed};
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

#[repr(C)]
pub struct CompressionOptions {
    pub jpeg_quality: i32,
    pub jpeg_chroma_subsampling: i32,
    pub jpeg_progressive: i32,
    pub png_quality: i32,
    pub png_optimization_level: i32,
    pub png_force_zopfli: i32,
    pub webp_quality: i32,
    pub tiff_compression: i32,
    pub tiff_deflate_level: i32,
    pub gif_quality: i32,
    pub keep_metadata: i32,
    pub optimize: i32,
    pub width: i32,
    pub height: i32,
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
pub unsafe extern fn w_compress(input: *const u8, input_size: usize, options: CompressionOptions) -> *mut c_void {
    let uncompressed_buffer = slice::from_raw_parts(input, input_size);
    handle_result(perform_compression(uncompressed_buffer, &options))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern fn w_compress_to_size(input: *const u8, input_size: usize, max_size: i32, options: CompressionOptions) -> *mut c_void {
    let uncompressed_buffer = slice::from_raw_parts(input, input_size);
    let keep_metadata = options.keep_metadata != 0;
    handle_result(perform_compression_to_size(uncompressed_buffer, max_size, keep_metadata))
}

fn handle_result(result: Result<Vec<u8>, CaesiumError>) -> *mut c_void {
    let mut status: u8 = 0;
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

fn perform_compression_to_size(file: &[u8], max_size: i32, keep_metadata: bool) -> Result<Vec<u8>, CaesiumError> {
    let mut parameters = CSParameters::new();
    parameters.keep_metadata = keep_metadata;
    let in_file = file.to_vec();
    caesium::compress_to_size_in_memory(in_file, &mut parameters, max_size as usize, true)
}

fn perform_compression(file: &[u8], options: &CompressionOptions) -> Result<Vec<u8>, CaesiumError> {
    let parameters = parse_options(options);
    let in_file = file.to_vec();
    caesium::compress_in_memory(in_file, &parameters)
}

fn parse_options(options: &CompressionOptions) -> CSParameters {
    let mut parameters = CSParameters::new();

    parameters.jpeg.quality = options.jpeg_quality.clamp(0, 100) as u32;
    parameters.jpeg.progressive = options.jpeg_progressive != 0;
    parameters.png.quality = options.png_quality.clamp(0, 100) as u32;
    parameters.optimize = options.optimize != 0;
    parameters.keep_metadata = options.keep_metadata != 0;
    parameters.png.optimization_level = options.png_optimization_level.clamp(0, 6) as u8;
    parameters.png.force_zopfli = options.png_force_zopfli != 0;
    parameters.gif.quality = options.gif_quality.clamp(0, 100) as u32;
    parameters.webp.quality = options.webp_quality.clamp(0, 100) as u32;
    parameters.width = options.width as u32;
    parameters.height = options.height as u32;

    parameters.jpeg.chroma_subsampling = match options.jpeg_chroma_subsampling {
        444 => ChromaSubsampling::CS444,
        422 => ChromaSubsampling::CS422,
        420 => ChromaSubsampling::CS420,
        411 => ChromaSubsampling::CS411,
        _ => ChromaSubsampling::Auto,
    };

    parameters.tiff.algorithm = match options.tiff_compression {
        1 => Lzw,
        2 => Deflate,
        3 => Packbits,
        _ => Uncompressed,
    };

    parameters.tiff.deflate_level = match options.tiff_deflate_level {
        1 => TiffDeflateLevel::Fast,
        6 => TiffDeflateLevel::Balanced,
        _ => TiffDeflateLevel::Best,
    };

    parameters
}