import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './views/pages/App.vue';
import { logger } from './services';
import { useErrorTracker } from './core/errorTracker';

function setupGlobalErrorHandlers() {
  window.addEventListener('error', (event) => {
    logger.error(`[window.onerror] ${event.message}`, {
      filename: event.filename,
      lineno: event.lineno,
      colno: event.colno,
    });
    return false;
  });

  window.addEventListener('unhandledrejection', (event) => {
    logger.error(`[unhandledrejection] ${event.reason}`, {});
  });
}

function logStartupInfo() {
  logger.info('=============================================');
  logger.info('Frontend starting...');
  logger.info('Version: 1.0.0');
  logger.info('=============================================');
}

// Initialize
const appEl = document.getElementById('app');
if (appEl) {
  appEl.innerHTML = '<div style="padding:20px;text-align:center;"><h2>Initializing...</h2></div>';
}

try {
  setupGlobalErrorHandlers();
  logStartupInfo();

  const app = createApp(App);
  const pinia = createPinia();
  
  app.use(pinia);
  app.mount('#app');

  (window as any).__errorTracker = useErrorTracker();
  logger.info('Application initialized successfully');
} catch (initError) {
  console.error('[FATAL INIT ERROR]', initError);
  const appEl = document.getElementById('app');
  if (appEl) {
    appEl.innerHTML = `<div style="padding:20px;color:red;background:#ffe6e6;"><h2>Fatal Error</h2><pre>${String(initError)}</pre></div>`;
  }
}
