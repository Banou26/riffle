import { builtinModules } from 'module'

import { defineConfig } from 'vite'

import pkg from './package.json'

export default defineConfig({
  build: {
    target: 'esnext',
    lib: {
      formats: ['es'],
      entry: ['./src/index.ts'],
      name: 'index'
    },
    sourcemap: true,
    minify: false,
    rollupOptions: {
      external: [
        ...builtinModules,
        ...Object.keys(pkg.dependencies)
      ]
    }
  }
})
