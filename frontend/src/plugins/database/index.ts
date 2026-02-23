import type { Plugin } from '../../core/plugin';

export interface User {
  id: number;
  name: string;
  email: string;
  role: string;
  status: string;
  created_at: string;
}

export interface DatabasePlugin extends Plugin {
  getUsers(): Promise<User[]>;
  getStats(): Promise<any>;
}

export const createDatabasePlugin = (): DatabasePlugin => {
  return {
    name: 'database',
    version: '1.0.0',

    initialize() {
      console.log('[DatabasePlugin] Initialized');
    },

    register() {
      console.log('[DatabasePlugin] Registered');
    },

    async getUsers(): Promise<User[]> {
      return new Promise((resolve, reject) => {
        const handler = (event: Event) => {
          const customEvent = event as CustomEvent;
          window.removeEventListener('db_response', handler);
          if (customEvent.detail.success) {
            resolve(customEvent.detail.data || []);
          } else {
            reject(new Error(customEvent.detail.error));
          }
        };

        window.addEventListener('db_response', handler);

        const webui = (window as any).webui;
        if (webui) {
          webui.call('get_users');
        } else {
          reject(new Error('WebUI not available'));
        }
      });
    },

    async getStats(): Promise<any> {
      return new Promise((resolve, reject) => {
        const handler = (event: Event) => {
          const customEvent = event as CustomEvent;
          window.removeEventListener('stats_response', handler);
          if (customEvent.detail.success) {
            resolve(customEvent.detail.stats);
          } else {
            reject(new Error(customEvent.detail.error));
          }
        };

        window.addEventListener('stats_response', handler);

        const webui = (window as any).webui;
        if (webui) {
          webui.call('get_db_stats');
        } else {
          reject(new Error('WebUI not available'));
        }
      });
    },
  };
};
