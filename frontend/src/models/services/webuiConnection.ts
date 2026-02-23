// WebUI communication - listens for CustomEvents from backend

const log = (...args) => console.log('[WebUI]', ...args);

class WebUIConnection {
  constructor() {
    this.handlers = new Map();
    this.currentPort = null;
    this.setupEventListeners();
    this.checkWebUI();
  }

  checkWebUI() {
    const check = () => {
      if (window.webui) {
        log('WebUI ready');
      } else {
        setTimeout(check, 100);
      }
    };
    check();
  }

  setupEventListeners() {
    // Listen for pong from backend
    window.addEventListener('backend_pong', (e) => {
      log('Pong received!', e.detail);
      this.triggerHandler('ping_backend', e.detail);
    });

    // Listen for window state responses
    window.addEventListener('window_state_response', (e) => {
      log('Window state response:', e.detail);
      this.triggerHandler('window_state_change', e.detail);
    });

    // Listen for history responses
    window.addEventListener('window_history_response', (e) => {
      log('History response:', e.detail);
      this.triggerHandler('get_window_state_history', e.detail);
    });

    // Listen for port info responses
    window.addEventListener('port_info_response', (e) => {
      log('Port info response:', e.detail);
      this.triggerHandler('port_info_request', e.detail);
    });
  }

  triggerHandler(eventName, data) {
    if (this.handlers.has(eventName)) {
      this.handlers.get(eventName)(data);
    }
  }

  on(eventName, callback) {
    this.handlers.set(eventName, callback);
  }

  async call(handler, data = '') {
    if (!window.webui) {
      log('WebUI not available');
      return null;
    }

    try {
      // Call the handler - backend will dispatch CustomEvent with response
      await window.webui.call(handler, data);
      return { success: true, message: 'Request sent' };
    } catch (e) {
      log('Call error:', e);
      return { success: false, error: e.message };
    }
  }

  async testConnection() {
    log('Testing connection...');
    return new Promise((resolve) => {
      // Set up one-time handler
      const handler = (e) => {
        log('Got pong!');
        this.handlers.delete('ping_test');
        resolve(true);
      };

      this.handlers.set('ping_test', handler);

      // Call ping
      this.call('ping_backend', '');

      // Timeout after 3 seconds
      setTimeout(() => {
        if (this.handlers.has('ping_test')) {
          this.handlers.delete('ping_test');
          resolve(false);
        }
      }, 3000);
    });
  }

  async getPortInfo() {
    log('Getting port information...');

    // Check if window.webui is available before calling
    if (!window.webui) {
      log('WebUI not available, cannot get port info');
      return {
        success: false,
        error: 'WebUI not available',
        port: 'unknown',
      };
    }

    return new Promise((resolve) => {
      // Set up one-time handler for port info
      const handler = (e) => {
        log('Got port info:', e.detail);
        this.handlers.delete('port_info_request');
        resolve(e.detail);
      };

      this.handlers.set('port_info_request', handler);

      // Call get_port_info and handle any errors
      this.call('get_port_info', '')
        .then((result) => {
          // If the call fails, the handler might not be triggered
          if (result && result.success === false) {
            log('get_port_info call failed:', result.error);
            this.handlers.delete('port_info_request');
            resolve({
              success: false,
              error: result.error || 'Call failed',
              port: 'unknown',
            });
          }
          // If successful, we wait for the response event to trigger the handler
        })
        .catch((error) => {
          log('Error calling get_port_info:', error);
          this.handlers.delete('port_info_request');
          resolve({
            success: false,
            error: error.message || 'Error calling get_port_info',
            port: 'unknown',
          });
        });

      // Timeout after 3 seconds
      setTimeout(() => {
        if (this.handlers.has('port_info_request')) {
          this.handlers.delete('port_info_request');
          resolve({
            success: false,
            error: 'Timeout getting port info',
            port: 'unknown',
          });
        }
      }, 3000);
    });
  }

  getStatus() {
    return {
      hasWebUI: !!window.webui,
      port: this.currentPort || 'dynamic',
    };
  }
}

export const webuiConn = new WebUIConnection();

export async function sendToBackend(handler, data = '') {
  return webuiConn.call(handler, data);
}

export async function testConnection() {
  return webuiConn.testConnection();
}

export async function getPortInfo() {
  return webuiConn.getPortInfo();
}

export function getConnectionStatus() {
  return webuiConn.getStatus();
}
