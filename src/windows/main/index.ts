import '@/assets/index.css';
import App from './App.vue';
import { createApp } from '@/lib/app';
import { handleError } from 'manatsu';
import { Route, router } from './router';
import { setupEventListeners } from './events';

const app = createApp(App).use(router);

setupEventListeners();

router
  .push({ name: Route.Home })
  .then(() => app.mount('#app'))
  .catch(handleError);
