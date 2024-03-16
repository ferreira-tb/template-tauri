import 'manatsu/components/style';
import '@manatsu/style/themes/mana';
import '@/assets/style.css';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createManatsu, registerComponents } from 'manatsu';
import App from './App.vue';
import { router } from './router';

const app = createApp(App);
const pinia = createPinia();
const manatsu = createManatsu();

app.use(router);
app.use(pinia);
app.use(manatsu);

registerComponents(app);

app.mount('#app');
