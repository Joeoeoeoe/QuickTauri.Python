import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath } from 'url';

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],
  resolve: {
    alias: {
      // 推荐的配置方式：'@' 映射到 'src' 目录
      '@': fileURLToPath(new URL('./src', import.meta.url))
      // 如果你还需要其他别名，可以继续添加，例如：
      // '@components': fileURLToPath(new URL('./src/components', import.meta.url)),
      // '@utils': fileURLToPath(new URL('./src/utils', import.meta.url)),
    }
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
    fs: {
      allow: ['.']
    }
  },
}));
