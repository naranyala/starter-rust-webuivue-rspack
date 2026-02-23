import { defineStore } from 'pinia';
import type { User, ApiResponse } from '../types';
import { webui } from '../services/webui';

interface UserState {
  users: User[];
  loading: boolean;
  error: string | null;
}

export const useUserStore = defineStore('user', {
  state: (): UserState => ({
    users: [],
    loading: false,
    error: null,
  }),

  getters: {
    getUserById: (state) => (id: number): User | undefined => {
      return state.users.find((user) => user.id === id);
    },
    activeUsers: (state): User[] => {
      return state.users.filter((user) => user.status === 'Active');
    },
  },

  actions: {
    async fetchUsers() {
      this.loading = true;
      this.error = null;

      try {
        const response = await webui.call<ApiResponse<User[]>>('get_users');
        if (response.success && response.data) {
          this.users = response.data;
        } else {
          this.error = response.error || 'Failed to fetch users';
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
      } finally {
        this.loading = false;
      }
    },

    async deleteUser(id: number) {
      this.loading = true;
      this.error = null;

      try {
        const response = await webui.call<ApiResponse>(`delete_user:${id}`);
        if (response.success) {
          this.users = this.users.filter((user) => user.id !== id);
        } else {
          this.error = response.error || 'Failed to delete user';
        }
        return response;
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
        return { success: false, error: this.error };
      } finally {
        this.loading = false;
      }
    },
  },
});
