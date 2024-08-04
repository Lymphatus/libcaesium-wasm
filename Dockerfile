FROM ghcr.io/cross-rs/wasm32-unknown-emscripten:latest

ENV EMCC_CFLAGS="-O3 -sALLOW_MEMORY_GROWTH=1 -sENVIRONMENT=web -sERROR_ON_UNDEFINED_SYMBOLS=0 -sMODULARIZE=1 -sEXPORT_NAME=CaesiumWASM -sEXPORT_ES6=1 -sEXPORTED_RUNTIME_METHODS=\"['cwrap', 'getValue']\""