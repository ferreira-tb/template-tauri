import '@/assets/index.css';
import App from './App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { RouteName, router } from './router';
import { setupEventListeners } from './events';
import { setupGlobalEventListeners } from '@/events';
import { createManatsu, handleError } from 'manatsu';

const app = createApp(App);
const pinia = createPinia();
const manatsu = createManatsu();

app.use(router);
app.use(pinia);
app.use(manatsu);

setupGlobalEventListeners();
setupEventListeners();

useColorMode().value = 'dark';

router
  .push({ name: RouteName.Home })
  .then(() => app.mount('#app'))
  .catch(handleError);
