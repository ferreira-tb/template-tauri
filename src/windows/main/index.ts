import App from './App.vue';
import { createApp } from '@/lib/app';
import { Route, router } from './router';
import { setupEventListeners } from './events';

const app = createApp(App).use(router);

setupEventListeners();

router
  .push({ name: Route.Home })
  .then(() => app.mount('#app'))
  .catch(handleError);
