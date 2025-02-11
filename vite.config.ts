import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import svgLoader from 'vite-svg-loader'
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  build: {
    rollupOptions: {
      output: {
        assetFileNames: (assetInfo) => {
            return 'assets/icons/[name][extname]'; // 保持 icons 目录结构
        }
      }
    }
  },
  plugins: [
    vue(),
    svgLoader(),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
