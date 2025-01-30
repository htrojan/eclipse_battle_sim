import {fileURLToPath, URL} from 'node:url'

import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
// import vueDevTools from 'vite-plugin-vue-devtools'
// import tailwindcss from 'tailwindcss'
import tailwindcss from "@tailwindcss/vite";
import wasm from 'vite-plugin-wasm'
import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        // vueDevTools(),
        wasm(),
        topLevelAwait(),
        tailwindcss(),
    ],
    base: '/eclipse_battle_sim/',
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        },
    },
    esbuild: {
        supported: {
            'top-level-await': true,
        }
    },
    worker: {
        plugins: () => {
            return [wasm(), topLevelAwait()]
        },
        format: 'es',
        // rollupOptions: {
        //   output: {
        //     inlineDynamicImports: true,
        //   },
        // },
    }
})
