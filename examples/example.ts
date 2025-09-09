import { compress, CompressionOptions, initialize } from '../dist/index';
import * as fs from 'node:fs';
import path from 'node:path';

// Get command line arguments
const args = process.argv.slice(2);

if (args.length === 0) {
  console.error('Usage: node example.js <path-to-image>');
  process.exit(1);
}

const filePath = args[0];

try {
  await initialize();

  if (!fs.existsSync(filePath)) {
    console.error(`File not found: ${filePath}`);
    process.exit(1);
  }

  console.log(`Compressing ${path.basename(filePath)}...`);
  const fileContent = fs.readFileSync(filePath);
  const compressionOptions: CompressionOptions = {
    jpeg: { quality: 50, chromaSubsampling: 0, progressive: true, optimize: false },
    png: { quality: 80, optimizationLevel: 2, forceZopfli: false, optimize: false },
    webp: { quality: 80, lossless: false },
    tiff: { compression: 80, deflateLevel: 6 },
    gif: { quality: 1 },
    keepMetadata: true,
    width: 0,
    height: 0,
  };
  const result = compress(new Uint8Array(fileContent), compressionOptions);

  if (result.status) {
    const parsedPath = path.parse(filePath);
    const outputPath = path.join(parsedPath.dir, `${parsedPath.name}_compressed${parsedPath.ext}`);

    // Write compressed image to file
    fs.writeFileSync(outputPath, result.compressedImage);

    const originalSize = fileContent.length;
    const compressedSize = result.size;
    const savings = (((originalSize - compressedSize) / originalSize) * 100).toFixed(2);

    console.log(`Original size: ${originalSize} bytes`);
    console.log(`Compressed size: ${compressedSize} bytes`);
    console.log(`Space savings: ${savings}%`);
    console.log(`Compressed image saved to: ${outputPath}`);
  } else {
    console.error(`Compression failed with error code: ${result.errorCode}`);
  }
} catch (error) {
  console.error(error);
}
