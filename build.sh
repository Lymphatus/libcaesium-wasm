export EMCC_CFLAGS="-s ALLOW_MEMORY_GROWTH=1 -s ENVIRONMENT=web -s ERROR_ON_UNDEFINED_SYMBOLS=0 -s EXPORTED_RUNTIME_METHODS=\"['cwrap', 'getValue']\""
cross clean
cross build --target wasm32-unknown-emscripten --release

#cp target/wasm32-unknown-emscripten/release/libcaesium-wasm.js static/libcaesium-wasm.js
#cp target/wasm32-unknown-emscripten/release/libcaesium_wasm.wasm static/libcaesium_wasm.wasm
