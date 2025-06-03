import {defineConfig} from 'vite';
import { viteStaticCopy } from 'vite-plugin-static-copy';


export default defineConfig({
    build: {
        lib: {
            entry: 'src/index.ts',
            name: 'CaesiumWASM',
            fileName: 'index',
            formats: ['es', 'cjs'],
        },
        rollupOptions: {
            output: {
                assetFileNames: '[name][extname]', // keep .wasm filename
            },
        },
        target: 'esnext',
        outDir: 'dist'
    },
    plugins: [
        viteStaticCopy({
            targets: [
                {
                    src: 'target/wasm32-unknown-emscripten/release/libcaesium_wasm.wasm',
                    dest: '' // copies directly into dist/
                }
            ]
        })
    ],
    assetsInclude: ['target/wasm32-unknown-emscripten/release/libcaesium_wasm.wasm'], // include WASM
});
