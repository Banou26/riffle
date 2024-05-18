import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    target: 'esnext',
    lib: {
      formats: ['es'],
      entry: ['./src/index.ts'],
      name: 'index'
    },
    sourcemap: true,
    minify: false
  }
})
