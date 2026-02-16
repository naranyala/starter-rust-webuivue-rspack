// webui-bridge.js
// This file provides the bridge between Vue.js frontend and Rust backend
import Logger from './logger.js';

class WebUIBridge {
  constructor() {
    this.callbacks = new Map();
    this.nextId = 1;
    this.logger = Logger;
  }

  // Call a Rust function
  callRustFunction(funcName, data = null) {
    return new Promise((resolve, reject) => {
      const id = this.nextId++;
      this.callbacks.set(id, { resolve, reject });

      // Log the function call
      this.logger.info(`Calling Rust function: ${funcName}`, {
        functionName: funcName,
        data: data,
        callId: id,
      });

      try {
        // Actual WebUI call to Rust backend
        if (window.__webui__) {
          // Call the Rust function using WebUI
          window.__webui__
            .call(funcName, JSON.stringify(data || {}))
            .then((result) => {
              this.logger.info(`Successfully called Rust function: ${funcName}`, {
                result: result,
                functionName: funcName,
              });
              resolve(JSON.parse(result));
            })
            .catch((error) => {
              this.logger.error(`Error calling Rust function ${funcName}: ${error.message}`, {
                functionName: funcName,
                error: error,
                data: data,
              });
              reject(error);
            });
        } else {
          // Fallback to simulated behavior if WebUI is not available
          this.logger.warn('WebUI not available, using simulated call', {
            functionName: funcName,
          });

          // Simulate different responses based on function name
          switch (funcName) {
            case 'open_folder':
              this.logger.info('Open folder operation completed successfully');
              resolve({
                success: true,
                path: '/home/user/images',
                images: [
                  { path: '/sample/image1.jpg', name: 'image1.jpg' },
                  { path: '/sample/image2.jpg', name: 'image2.jpg' },
                  { path: '/sample/image3.jpg', name: 'image3.jpg' },
                ],
              });
              break;
            case 'organize_images':
              this.logger.info('Images organized successfully');
              resolve({ success: true, message: 'Images organized successfully!' });
              break;
            case 'increment_counter':
              this.logger.debug(`Counter incremented to ${data?.value || 'unknown'}`, {
                value: data?.value,
                functionName: funcName,
              });
              resolve({ success: true, value: data?.value || 0 });
              break;
            case 'reset_counter':
              this.logger.debug(`Counter reset to ${data?.value || 'unknown'}`, {
                value: data?.value,
                functionName: funcName,
              });
              resolve({ success: true, value: data?.value || 0 });
              break;
            default:
              this.logger.warn(`Unknown function called: ${funcName}`);
              resolve({ success: true });
          }
        }
      } catch (error) {
        this.logger.error(`Error in Rust function call: ${error.message}`, {
          functionName: funcName,
          error: error,
          data: data,
        });
        reject(error);
      }
    });
  }

  // Handle responses from Rust (this would be called by the actual webui library)
  handleResponse(response) {
    // This method would be called when Rust sends a response back to the frontend
    this.logger.info('Received response from Rust backend', { response });
  }
}

// Create a global instance
window.WebUIBridge = new WebUIBridge();

// Export for use in Vue components
export default window.WebUIBridge;
