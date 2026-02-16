import { createApp } from 'vue';
import App from './views/pages/App.vue';
import './services/webui-bridge.js';

const app = createApp(App);
app.mount('#app');
