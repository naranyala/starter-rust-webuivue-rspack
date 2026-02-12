import { createApp } from 'vue';
import App from './use-cases/App.vue';
import './lib/webui-bridge.js';

const app = createApp(App);
app.mount('#app');