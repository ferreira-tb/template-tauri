import '@/assets/style.css';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createManatsu, handleError } from 'manatsu';
import App from './App.vue';
import { RouteName, router } from './router';

const app = createApp(App);
const pinia = createPinia();
const manatsu = createManatsu();

app.use(router);
app.use(pinia);
app.use(manatsu);

router
  .push({ name: RouteName.Home })
  .then(() => app.mount('#app'))
  .catch(handleError);
