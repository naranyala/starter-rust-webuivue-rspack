import { computed, onMounted, ref } from 'vue';
import { useUserStore } from '../models/stores';
import type { User } from '../models/types';

export function useUserListViewModel() {
  const userStore = useUserStore();
  const showAddUserModal = ref(false);
  const searchTerm = ref('');

  const currentUser = ref<Partial<User>>({
    id: null,
    name: '',
    email: '',
    role: '',
    status: '',
    department: '',
    lastLogin: new Date().toISOString(),
  });

  const filteredUsers = computed(() => {
    if (!searchTerm.value) return userStore.users;
    const term = searchTerm.value.toLowerCase();
    return userStore.users.filter(
      (user) =>
        user.name.toLowerCase().includes(term) ||
        user.email.toLowerCase().includes(term) ||
        user.role.toLowerCase().includes(term) ||
        user.status.toLowerCase().includes(term)
    );
  });

  const activeUsersCount = computed(
    () => userStore.users.filter((user) => user.status === 'Active').length
  );
  const adminUsersCount = computed(
    () => userStore.users.filter((user) => user.role === 'Admin').length
  );

  const loadUsers = async () => {
    await userStore.fetchUsers();
  };

  const editUser = (user: User) => {
    currentUser.value = { ...user };
    showAddUserModal.value = true;
  };

  const deleteUser = async (id: number) => {
    if (confirm('Are you sure you want to delete this user?')) {
      await userStore.deleteUser(id);
    }
  };

  const saveUser = async () => {
    try {
      if (currentUser.value.id) {
        await userStore.updateUser(currentUser.value.id, currentUser.value);
      } else {
        await userStore.createUser(currentUser.value);
      }
      closeModal();
    } catch (err) {
      console.error('Error saving user:', err);
    }
  };

  const closeModal = () => {
    showAddUserModal.value = false;
    currentUser.value = {
      id: null,
      name: '',
      email: '',
      role: '',
      status: '',
      department: '',
      lastLogin: new Date().toISOString(),
    };
  };

  const getUserInitials = (name: string): string => {
    return name
      .split(' ')
      .map((n) => n.charAt(0))
      .join('')
      .toUpperCase()
      .substring(0, 2);
  };

  const formatDate = (dateString?: string): string => {
    if (!dateString) return 'Never';
    const date = new Date(dateString);
    return (
      date.toLocaleDateString() +
      ' ' +
      date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
    );
  };

  onMounted(() => {
    loadUsers();
  });

  return {
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
    loadUsers,
  };
}
