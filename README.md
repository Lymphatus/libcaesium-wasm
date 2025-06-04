This allows you to run (almost) the full libcaesium potential directly inside the browser or in any node.js environment.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building the Project](#building-the-project)
- [Usage](#usage)
    - [Node.js Example](#nodejs-example)
    - [Browser Example](#browser-example)
- [API Reference](#api-reference)
- [License](#license)

## Prerequisites

Before building this project, make sure you have the following tools installed:

### Required Tools

- [**Rust**](https://www.rust-lang.org/) (Latest stable version recommended)
- [**Docker**](https://www.docker.com/) (or any alternative you like)
- [**Cross**](https://github.com/cross-rs/cross) (For cross-compilation to WASM)
- [**Node.js**](https://nodejs.org/) (v22 or higher recommended)

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/Lymphatus/libcaesium-wasm.git
   cd libcaesium-wasm
   ```

2. Install Node.js dependencies:
   ```sh
   npm install
   ```

## Building the Project

To build both the Rust WASM library and TypeScript bindings:

```sh
npm run build
``` 

This command will:

1. Compile the Rust code to WASM using Cross and Docker (`npm run build:rust`)
2. Build the TypeScript wrapper and generate type definitions (`npm run build:ts`)

### Individual Build Steps

You can also run the build steps individually:

```sh
# Build only the Rust WASM library
npm run build:rust
# Build only the TypeScript wrapper
npm run build:ts
``` 

### Build Output

After a successful build, you'll find the output files in the `dist` directory.

## Usage

### Node.js Example

You can test the provided example by running:

```sh
# Build the project first
npm run build
# Run the example with an image file
npx tsx examples/example.ts path/to/your/image.jpg
``` 

This will output the compressed image in the same folder as the original with the `_compressed` suffix.

### Browser Example

To test the browser example:

1. Build the project:
   ```sh
   npm run build
   ```

2. Serve the `index.html` file with a local web server:

   ```sh
   # Using Python
   python -m http.server 3000
   
   # Or using Node.js
   npx serve .
   ```

3. Open `http://localhost:3000` in your browser

## API Reference

### Functions

- `initialize()`: Promise that initializes the WASM module. Must be called before using compression functions.
- `compress(imageData: Uint8Array, options: CompressionOptions)`: Compresses an image with the specified options.

### Types

```typescript
interface CompressionOptions {
    jpeg: { quality: number; chromaSubsampling: number; progressive: boolean };
    png: { quality: number; optimizationLevel: number; forceZopfli: boolean };
    webp: { quality: number };
    tiff: { compression: number; deflateLevel: number };
    gif: { quality: number };
    keepMetadata: boolean;
    optimize: boolean;
    width: number;
    height: number;
}
```

## License

See LICENSE.md for details.
