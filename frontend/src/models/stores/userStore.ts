import { defineStore } from 'pinia';
import type { User } from '../types';

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
    getUserById:
      (state) =>
      (id: number): User | undefined => {
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
        const result = await window.webui.call('get_users');
        const response = JSON.parse(result);

        if (response.success) {
          this.users = response.data;
        } else {
          this.error = response.error;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
      } finally {
        this.loading = false;
      }
    },

    async createUser(userData: Partial<User>) {
      this.loading = true;
      this.error = null;

      try {
        const elementName = `create_user:${userData.name}:${userData.email}:${userData.role}:${userData.status}`;
        const result = await window.webui.call(elementName);
        const response = JSON.parse(result);

        if (response.success) {
          await this.fetchUsers();
          return response;
        } else {
          this.error = response.error;
          return response;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
        return { success: false, error: this.error };
      } finally {
        this.loading = false;
      }
    },

    async updateUser(id: number, userData: Partial<User>) {
      this.loading = true;
      this.error = null;

      try {
        const elementName = `update_user:${id}:${userData.name || ''}:${userData.email || ''}:${userData.role || ''}:${userData.status || ''}`;
        const result = await window.webui.call(elementName);
        const response = JSON.parse(result);

        if (response.success) {
          await this.fetchUsers();
          return response;
        } else {
          this.error = response.error;
          return response;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
        return { success: false, error: this.error };
      } finally {
        this.loading = false;
      }
    },

    async deleteUser(id: number) {
      this.loading = true;
      this.error = null;

      try {
        const elementName = `delete_user:${id}`;
        const result = await window.webui.call(elementName);
        const response = JSON.parse(result);

        if (response.success) {
          this.users = this.users.filter((user) => user.id !== id);
          return response;
        } else {
          this.error = response.error;
          return response;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
        return { success: false, error: this.error };
      } finally {
        this.loading = false;
      }
    },
  },
});
