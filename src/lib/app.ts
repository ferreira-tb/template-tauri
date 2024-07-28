import '@/assets/index.css';
import { createPinia } from 'pinia';
import { createManatsu } from 'manatsu';
import { createApp as createVue } from 'vue';
import { createPlugin } from 'tauri-plugin-pinia';
import { setupGlobalEventListeners } from '@/events';

export function createApp(root: Component) {
  const app = createVue(root);
  const pinia = createPinia();
  const manatsu = createManatsu();

  pinia.use(
    createPlugin({
      onError: handleError,
    })
  );

  app.use(pinia);
  app.use(manatsu);

  setupGlobalEventListeners();

  return app;
}
