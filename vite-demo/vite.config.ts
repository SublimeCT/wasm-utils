import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'

// https://vite.dev/config/
export default defineConfig({
  server: { port: 53311 },
  plugins: [vue(), wasm()],
  optimizeDeps: {
    exclude: [
      'wasm-utils'
    ]
  }
})
