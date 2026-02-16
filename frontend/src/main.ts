import { createApp } from 'vue';
import App from './views/pages/App.vue';
import './services/webui-bridge.js';

const logger = (window as any).logger;
if (logger) {
  logger.info('=============================================');
  logger.info('Frontend: Communication Options');
  logger.info('');
  logger.info('[Transport Layer]:');
  logger.info('  1. WebUI IPC    - Direct IPC via window.__webui__');
  logger.info('  2. WebSocket    - TCP real-time (check backend for port)');
  logger.info('  3. HTTP/REST    - Available for REST API calls');
  logger.info('');
  logger.info('[Serialization Format]:');
  logger.info('  1. JSON         - Human-readable, widely supported (current)');
  logger.info('  2. MessagePack  - Binary, compact, fast (msgpack-lite)');
  logger.info('  3. CBOR         - Binary, self-describing (cbor-js)');
  logger.info('  4. UBJSON       - Binary, type-safe (ubjson)');
  logger.info('=============================================');
  logger.info('Selected: WebUI IPC + WebSocket (hybrid mode)');
  logger.info('  - Transport: WebUI IPC + WebSocket');
  logger.info('  - Serialization: JSON');
  logger.info('=============================================');
}

const app = createApp(App);
app.mount('#app');
