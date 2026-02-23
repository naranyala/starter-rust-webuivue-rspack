import type { Plugin } from '../../core/plugin';

export interface HomePlugin extends Plugin {
  openSystemInfoWindow(): void;
  openSQLiteWindow(): void;
}

export const createHomePlugin = (): HomePlugin => {
  return {
    name: 'home',
    version: '1.0.0',

    initialize() {
      console.log('[HomePlugin] Initialized');
    },

    register() {
      console.log('[HomePlugin] Registered');
    },

    openSystemInfoWindow() {
      console.log('[HomePlugin] Opening system info window');
    },

    openSQLiteWindow() {
      console.log('[HomePlugin] Opening SQLite window');
    },
  };
};
