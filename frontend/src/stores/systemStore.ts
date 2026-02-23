import { defineStore } from 'pinia';
import type { SystemInfo, ApiResponse } from '../types';
import { webui } from '../services/webui';

interface SystemState {
  info: SystemInfo | null;
  loading: boolean;
  error: string | null;
}

export const useSystemStore = defineStore('system', {
  state: (): SystemState => ({
    info: null,
    loading: false,
    error: null,
  }),

  getters: {
    isLinux: (state): boolean => {
      return state.info?.platform?.toLowerCase().includes('linux') ?? false;
    },
  },

  actions: {
    async fetchSystemInfo() {
      this.loading = true;
      this.error = null;

      try {
        const response = await webui.call<ApiResponse<SystemInfo>>('get_system_info');
        if (response.success && response.data) {
          this.info = response.data;
        } else {
          this.error = response.error || 'Failed to fetch system info';
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
      } finally {
        this.loading = false;
      }
    },
  },
});
