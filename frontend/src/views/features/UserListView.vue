<template>
  <div class="user-list">
    <div class="user-list-header">
      <h2>👥 User Management</h2>
      <div class="header-actions">
        <input v-model="searchTerm" type="text" placeholder="Search users..." class="search-input" />
        <Button variant="primary" @click="showAddUserModal = true">+ Add User</Button>
      </div>
    </div>

    <div v-if="userStore.loading" class="loading">Loading users...</div>
    <div v-else-if="userStore.error" class="error">{{ userStore.error }}</div>
    <div v-else class="user-content">
      <div class="content-wrapper">
        <div class="article-content">
          <h3>About User Management</h3>
          <p>User management is a critical aspect of any application that involves multiple users. Effective user management ensures that the right people have access to the right resources at the right times.</p>
          <p>Modern user management systems typically include features such as role-based access control, user authentication, and authorization mechanisms.</p>
          <p>Best practices include regular audits of user permissions, implementing strong password policies, and using multi-factor authentication.</p>
        </div>
        
        <div class="stats-summary">
          <div class="stat-card">
            <div class="stat-number">{{ filteredUsers.length }}</div>
            <div class="stat-label">Total Users</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">{{ activeUsersCount }}</div>
            <div class="stat-label">Active</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">{{ adminUsersCount }}</div>
            <div class="stat-label">Admins</div>
          </div>
        </div>

        <div class="user-table-container">
          <table class="user-table">
            <thead>
              <tr><th>ID</th><th>Name</th><th>Email</th><th>Role</th><th>Status</th><th>Last Login</th><th>Actions</th></tr>
            </thead>
            <tbody>
              <tr v-for="user in filteredUsers" :key="user.id">
                <td>{{ user.id }}</td>
                <td>
                  <div class="user-info">
                    <div class="user-avatar">{{ getUserInitials(user.name) }}</div>
                    <div>{{ user.name }}</div>
                  </div>
                </td>
                <td>{{ user.email }}</td>
                <td><span :class="['badge', `badge-${user.role.toLowerCase()}`]">{{ user.role }}</span></td>
                <td><span :class="['badge', `badge-${user.status.toLowerCase()}`]">{{ user.status }}</span></td>
                <td>{{ formatDate(user.lastLogin) }}</td>
                <td>
                  <Button variant="secondary" @click="editUser(user)" size="small">Edit</Button>
                  <Button variant="danger" @click="deleteUser(user.id)" size="small">Delete</Button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <div v-if="showAddUserModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3 class="modal-title">{{ currentUser.id ? 'Edit User' : 'Add New User' }}</h3>
          <button class="modal-close" @click="closeModal">&times;</button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="saveUser">
            <div class="form-row">
              <div class="form-group">
                <label>Name</label>
                <input v-model="currentUser.name" type="text" placeholder="Enter full name" required />
              </div>
              <div class="form-group">
                <label>Email</label>
                <input v-model="currentUser.email" type="email" placeholder="Enter email address" required />
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>Role</label>
                <select v-model="currentUser.role" required>
                  <option value="Admin">Admin</option>
                  <option value="Editor">Editor</option>
                  <option value="User">User</option>
                </select>
              </div>
              <div class="form-group">
                <label>Status</label>
                <select v-model="currentUser.status" required>
                  <option value="Active">Active</option>
                  <option value="Inactive">Inactive</option>
                  <option value="Pending">Pending</option>
                </select>
              </div>
            </div>
            <div class="form-group">
              <label>Department</label>
              <input v-model="currentUser.department" type="text" placeholder="Enter department" />
            </div>
            <div class="modal-footer">
              <Button variant="secondary" @click="closeModal">Cancel</Button>
              <Button variant="primary" :loading="userStore.loading">{{ currentUser.id ? 'Update' : 'Create' }} User</Button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import Button from '../../components/common/Button.vue';
import { useUserListViewModel } from '../../viewmodels/useUserListViewModel.ts';

const {
  userStore,
  showAddUserModal,
  searchTerm,
  currentUser,
  filteredUsers,
  activeUsersCount,
  adminUsersCount,
  editUser,
  deleteUser,
  saveUser,
  closeModal,
  getUserInitials,
  formatDate,
} = useUserListViewModel();
</script>

<style scoped>
.user-list { padding: 1rem; height: 100%; display: flex; flex-direction: column; }
.user-list-header { display: flex; flex-direction: column; gap: 1rem; margin-bottom: 1rem; }
.header-actions { display: flex; gap: 0.5rem; }
.search-input { flex: 1; padding: 0.5rem 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem; }
.stats-summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 1rem; margin-bottom: 1.5rem; }
.stat-card { background: white; border-radius: 0.5rem; padding: 1rem; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.stat-number { font-size: 1.5rem; font-weight: 700; color: #4f46e5; margin-bottom: 0.25rem; }
.stat-label { font-size: 0.875rem; color: #6b7280; }
.user-content { flex: 1; display: flex; flex-direction: column; }
.user-table-container { flex: 1; overflow-x: auto; min-height: 0; }
.user-table { width: 100%; border-collapse: collapse; background: white; border-radius: 0.5rem; overflow: hidden; box-shadow: 0 1px 3px rgba(0,0,0,0.1); min-width: 800px; }
.user-table th, .user-table td { padding: 0.75rem; text-align: left; border-bottom: 1px solid #e5e7eb; }
.user-table th { background-color: #f3f4f6; font-weight: 600; color: #374151; position: sticky; top: 0; }
.user-info { display: flex; align-items: center; gap: 0.5rem; }
.user-avatar { width: 32px; height: 32px; border-radius: 50%; background: #4f46e5; color: white; display: flex; align-items: center; justify-content: center; font-size: 0.75rem; font-weight: 600; }
.badge { padding: 0.25rem 0.5rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; display: inline-block; }
.badge-admin { background-color: #fee2e2; color: #dc2626; }
.badge-user { background-color: #dbeafe; color: #2563eb; }
.badge-editor { background-color: #fef3c7; color: #d97706; }
.badge-active { background-color: #dcfce7; color: #16a34a; }
.badge-inactive { background-color: #f3f4f6; color: #6b7280; }
.badge-pending { background-color: #fffbeb; color: #f59e0b; }
.loading, .error { padding: 1rem; text-align: center; }
.error { color: #dc2626; background-color: #fee2e2; border-radius: 0.5rem; margin-bottom: 1rem; }
.modal-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.modal-content { background: white; border-radius: 0.5rem; width: 100%; max-width: 600px; max-height: 90vh; overflow-y: auto; }
.modal-header { display: flex; justify-content: space-between; align-items: center; padding: 1rem; border-bottom: 1px solid #e5e7eb; }
.modal-title { font-size: 1.25rem; font-weight: 600; margin: 0; }
.modal-close { background: none; border: none; font-size: 1.5rem; cursor: pointer; }
.modal-body { padding: 1rem; }
.form-row { display: flex; gap: 1rem; margin-bottom: 1rem; }
.form-row .form-group { flex: 1; }
.form-group { margin-bottom: 1rem; }
.form-group label { display: block; margin-bottom: 0.5rem; font-weight: 500; }
.form-group input, .form-group select { width: 100%; padding: 0.5rem; border: 1px solid #e5e7eb; border-radius: 0.375rem; }
.modal-footer { display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem; }
.article-content { background: white; border-radius: 0.5rem; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.article-content h3 { color: #4f46e5; margin-top: 0; margin-bottom: 1rem; font-size: 1.25rem; }
.article-content p { margin-bottom: 1rem; line-height: 1.6; color: #4b5563; }
.content-wrapper { display: flex; flex-direction: column; gap: 1.5rem; }
</style>
