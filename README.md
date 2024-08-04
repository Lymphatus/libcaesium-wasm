# Libcaesium WASM

WASM compiled code for [libcaesium](https://github.com/Lymphatus/libcaesium).  
This allows you to run (almost) the full libcaesium potential directly inside the browser.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

### Prerequisites

- [Docker](https://www.docker.com/get-started)
- [Cross](https://github.com/cross-rs/cross)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/Lymphatus/libcaesium-wasm.git
    cd libcaesium-wasm
    ```

2. Install Cross:

    ```sh
    cargo install cross --git https://github.com/cross-rs/cross
    ```

3. Build the library:

   ```sh
   cross build --release
   ```

### Usage

You can test the compression by copying the compiled `libcaesium-wasm.js` and `libcaesium_wasm.wasm` files into
the `static` folder of the project. You can then open the `static/index.html` file to have a small demo of the usage.

The copy command should look like this

   ```sh
   cp target/wasm32-unknown-emscripten/release/libcaesium-wasm.js static/libcaesium-wasm.js
   cp target/wasm32-unknown-emscripten/release/libcaesium_wasm.wasm static/libcaesium_wasm.wasm
   ```

Note that the code by default is compiled as an ES6 Module, so it supports only modern browsers.  
The library can be compiled as vanilla JS or UMD by tweaking the `EMCC_CFLAGS` inside `Dockerfile`.
### License

Licensed under Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
See LICENSE.md for details.
