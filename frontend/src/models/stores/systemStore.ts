import { defineStore } from 'pinia';
import { wsManager } from '../services/websocket';
import type { SystemInfo } from '../types';

interface SystemState {
  systemInfo: SystemInfo | null;
  uptime: number;
  loading: boolean;
  error: string | null;
}

export const useSystemStore = defineStore('system', {
  state: (): SystemState => ({
    systemInfo: null,
    uptime: 0,
    loading: false,
    error: null,
  }),

  getters: {
    isSystemReady: (state): boolean => !!state.systemInfo,
  },

  actions: {
    async fetchSystemInfo() {
      this.loading = true;
      this.error = null;

      try {
        // Use WebSocket to send request
        const response = await this.sendRequest('get_system_info');

        if (response.success) {
          this.systemInfo = response.data;
        } else {
          this.error = response.error || 'Failed to get system info';
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
      } finally {
        this.loading = false;
      }
    },

    async fetchUptime() {
      this.loading = true;
      this.error = null;

      try {
        // Use WebSocket to send request
        const response = await this.sendRequest('get_uptime');

        if (response.success) {
          this.uptime = response.uptime_seconds;
        } else {
          this.error = response.error || 'Failed to get uptime';
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
      } finally {
        this.loading = false;
      }
    },

    // Helper method to send requests via WebSocket and wait for response
    sendRequest(type: string): Promise<any> {
      return new Promise((resolve, reject) => {
        // Generate a unique request ID
        const requestId = Date.now().toString() + Math.random().toString(36).substr(2, 9);

        // Create the request payload
        const requestPayload = {
          type: 'request',
          request_type: type,
          request_id: requestId,
          timestamp: Date.now(),
          data: null,
        };

        // Set up response listener
        let unsubscribe: (() => void) | null = null;
        const responseHandler = (data: any) => {
          if (data.request_id === requestId) {
            if (unsubscribe) unsubscribe(); // Remove the listener
            resolve(data);
          }
        };

        // Set up timeout
        const timeoutId = setTimeout(() => {
          if (unsubscribe) unsubscribe(); // Remove the listener
          reject(new Error('Request timeout'));
        }, 10000); // 10 second timeout

        // Add event listener for response
        unsubscribe = wsManager.onMessage(responseHandler);

        // Send the request via WebSocket
        wsManager
          .send(requestPayload)
          .then((success) => {
            if (!success) {
              clearTimeout(timeoutId);
              unsubscribe(); // Remove the listener
              reject(new Error('Failed to send request via WebSocket'));
            }
          })
          .catch((err) => {
            clearTimeout(timeoutId);
            unsubscribe(); // Remove the listener
            reject(err);
          });
      });
    },
  },
});
