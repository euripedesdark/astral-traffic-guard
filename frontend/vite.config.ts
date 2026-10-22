import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  base: '/proxy/',
  build: {
    outDir: '../src/main/resources/static/proxy',
    emptyOutDir: true
  }
});
