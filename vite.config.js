import { URL, fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import dev from 'vite-plugin-vue-devtools';
import autoImport from 'unplugin-auto-import/vite';
import components from 'unplugin-vue-components/vite';
import componentsConfig from '@tb-dev/vue-import-config';
import autoImportConfig from '@tb-dev/auto-import-config';

const autoImportOptions = autoImportConfig({
  manatsu: true,
  imports: [
    {
      '@/utils': ['useInvoke']
    }
  ]
});

export default defineConfig({
  plugins: [vue(), dev(), autoImport(autoImportOptions), components(componentsConfig())],
  clearScreen: false,
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('src', import.meta.url))
    }
  },
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**']
    }
  },
  build: {
    emptyOutDir: true,
    minify: false
  }
});
