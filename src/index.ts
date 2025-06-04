import CaesiumWASM from 'libcaesium-wasm.js';

export interface ILibcaesium {
  _malloc: (size: number) => number;
  _free: (ptr: number) => void;
  HEAP8: Int8Array;
  HEAPU8: Uint8Array;
  HEAP16: Int16Array;
  HEAPU16: Uint16Array;
  HEAP32: Int32Array;
  HEAPU32: Uint32Array;
  HEAPF32: Float32Array;
  HEAPF64: Float64Array;
  getValue: (ptr: number, type: string) => number;
  cwrap: (name: string, returnType: unknown, argTypes: string[]) => (...args: unknown[]) => number;
  ready: Promise<void>;
  asm: {
    za: WebAssembly.Memory;
    Fa: WebAssembly.Table;
    Aa: (...args: unknown[]) => unknown;
  };
}

let libcaesium: ILibcaesium | null = null;

export async function initialize(): Promise<ILibcaesium | null> {
  if (!libcaesium) {
    libcaesium = await CaesiumWASM();
    await libcaesium?.ready;
  }
  return libcaesium;
}

export interface CompressionResult {
  status: boolean;
  errorCode: number;
  compressedImage: Uint8Array;
  size: number;
}

export interface CompressionOptions {
  jpeg: {
    quality: number;
    chromaSubsampling: number;
    progressive: boolean;
  };
  png: {
    quality: number;
    optimizationLevel: number;
    forceZopfli: boolean;
  };
  webp: {
    quality: number;
  };
  tiff: {
    compression: number;
    deflateLevel: number;
  };
  gif: {
    quality: number;
  };
  keepMetadata: boolean;
  optimize: boolean;
  width: number;
  height: number;
}

const clamp = (val: number, min: number, max: number) => Math.min(Math.max(val, min), max);

export function compress(ui8a: Uint8Array, options: CompressionOptions): CompressionResult {
  if (!libcaesium) {
    throw new Error('libcaesium is not initialized.');
  }

  const ptr = libcaesium._malloc(ui8a.length);
  libcaesium.HEAP8.set(ui8a, ptr);

  const buffer = new ArrayBuffer(4 * 14);
  const view = new DataView(buffer);

  //TODO what if we exceed the size of each option?
  view.setInt32(0, clamp(options.jpeg.quality, 0, 100), true); // jpeg_quality
  view.setInt32(4, options.jpeg.chromaSubsampling, true); // jpeg_chroma_subsampling
  view.setInt32(8, options.jpeg.progressive ? 1 : 0, true); // jpeg_progressive
  view.setInt32(12, clamp(options.png.quality, 0, 100), true); // png_quality
  view.setInt32(16, clamp(options.png.optimizationLevel, 0, 6), true); // png_optimization_level
  view.setInt32(20, options.png.forceZopfli ? 1 : 0, true); // png_force_zopfli
  view.setInt32(24, clamp(options.webp.quality, 0, 100), true); // webp_quality
  view.setInt32(28, clamp(options.tiff.compression, 1, 4), true); // tiff_compression //TODO enum
  view.setInt32(32, clamp(options.tiff.deflateLevel, 1, 9), true); // tiff_deflate_level
  view.setInt32(36, clamp(options.gif.quality, 1, 4), true); // gif_quality
  view.setInt32(40, options.keepMetadata ? 1 : 0, true); // keep_metadata
  view.setInt32(44, options.optimize ? 1 : 0, true); // optimize
  view.setInt32(48, options.width, true); // width
  view.setInt32(52, options.height, true); // height

  const opts_ptr = libcaesium._malloc(buffer.byteLength);
  libcaesium.HEAPU8.set(new Uint8Array(buffer), opts_ptr);

  const js_wrapped_compress = libcaesium.cwrap('w_compress', 'number', ['number', 'number', 'number']);
  const dataPtr = js_wrapped_compress(ptr, ui8a.length, opts_ptr);
  const status = libcaesium.getValue(dataPtr, 'i32');
  const errorCode = libcaesium.getValue(dataPtr + 4, 'i32');
  const vecPtr = libcaesium.getValue(dataPtr + 8, 'i32');
  const len = libcaesium.getValue(dataPtr + 12, 'i32');

  const res: CompressionResult = {
    status: status === 1,
    errorCode,
    compressedImage: new Uint8Array(libcaesium.HEAPU8.buffer, vecPtr, len),
    size: len,
  };

  const drop_vector_struct = libcaesium.cwrap('drop_vector_struct', null, ['number']);
  drop_vector_struct(dataPtr);

  libcaesium._free(ptr);

  return res;
}
