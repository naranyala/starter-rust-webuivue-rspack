import { computed, ref } from 'vue';
import type { CardItem } from '../models/types';

export function useHomeViewModel() {
  const searchQuery = ref('');

  const cards: CardItem[] = [
    {
      id: 'eventlog',
      title: 'Event Log',
      description: 'View real-time application events',
      icon: '📡',
      component: 'EventLog',
    },
    {
      id: 'windowtracker',
      title: 'Window Tracker',
      description: 'Monitor window state changes and backend logging',
      icon: '🪟',
      component: 'WindowTracker',
    },
    {
      id: 'article',
      title: 'Article',
      description: 'Read the latest article',
      icon: '📄',
      component: 'ArticleView',
    },
    {
      id: 'users',
      title: 'User Management',
      description: 'Manage users, roles, and permissions',
      icon: '👥',
      component: 'UserList',
    },
    {
      id: 'products',
      title: 'Product Management',
      description: 'Manage products, inventory, and pricing',
      icon: '📦',
      component: 'ProductList',
    },
    {
      id: 'system',
      title: 'System Information',
      description: 'View system details and diagnostics',
      icon: '[System]',
      component: 'SystemInfo',
    },
  ];

  const filteredCards = computed(() => {
    if (!searchQuery.value) return cards;
    const query = searchQuery.value.toLowerCase();
    return cards.filter(
      (card) =>
        card.title.toLowerCase().includes(query) || card.description.toLowerCase().includes(query)
    );
  });

  const openWindow = (card: CardItem) => {
    const event = new CustomEvent('open-window', { detail: card });
    window.dispatchEvent(event);
  };

  const triggerDemoError = () => {
    window.showError({
      title: 'Demo Error',
      message: 'This is a demonstration of the error handling system.',
      stack: `Error: Demo Error\n    at triggerDemoError (HomePanel.vue:95:15)`,
      context: 'HomePanel.vue - Demo Button',
    });
  };

  return {
    searchQuery,
    cards,
    filteredCards,
    openWindow,
    triggerDemoError,
  };
}
