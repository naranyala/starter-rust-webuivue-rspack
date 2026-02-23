import type { Plugin } from '../../core/plugin';

export interface SystemInfoPlugin extends Plugin {
  getSystemInfo(): Promise<any>;
}

export const createSystemInfoPlugin = (): SystemInfoPlugin => {
  return {
    name: 'system-info',
    version: '1.0.0',

    initialize() {
      console.log('[SystemInfoPlugin] Initialized');
    },

    register() {
      console.log('[SystemInfoPlugin] Registered');
    },

    async getSystemInfo() {
      return new Promise((resolve, reject) => {
        const handler = (event: Event) => {
          const customEvent = event as CustomEvent;
          window.removeEventListener('sysinfo_response', handler);
          resolve(customEvent.detail);
        };

        window.addEventListener('sysinfo_response', handler);

        const webui = (window as any).webui;
        if (webui) {
          webui.call('get_system_info');
        } else {
          reject(new Error('WebUI not available'));
        }
      });
    },
  };
};
