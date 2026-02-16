<template>
  <aside class="sidebar">
    <div class="home-button-container">
      <button @click="$emit('home')" class="home-btn" title="Show Main View">
        <span class="home-icon">🏠</span>
        <span class="home-text">Home</span>
      </button>
    </div>

    <div class="sidebar-header">
      <h2>Windows</h2>
      <span class="window-count">{{ windows.length }}</span>
    </div>

    <div class="window-list">
      <div
        v-for="window in windows"
        :key="window.id"
        class="window-item"
        :class="{ minimized: window.minimized }"
        @click="$emit('toggle', window)"
      >
        <div class="window-icon">📷</div>
        <div class="window-info">
          <span class="window-title">{{ window.title }}</span>
          <span class="window-status">{{ window.minimized ? 'Minimized' : 'Active' }}</span>
        </div>
        <button
          class="window-close"
          @click.stop="$emit('close', window)"
          title="Close window"
        >×</button>
      </div>

      <div v-if="windows.length === 0" class="no-windows">
        No open windows
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { WindowInfo } from '../composables/useWindowManager';

defineProps<{
  windows: WindowInfo[];
}>();

defineEmits<{
  home: [];
  toggle: [window: WindowInfo];
  close: [window: WindowInfo];
}>();
</script>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
  color: white;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #334155;
  flex-shrink: 0;
}

.home-button-container {
  padding: 0.75rem;
  background: rgba(79, 70, 229, 0.2);
  border-bottom: 1px solid #334155;
}

.home-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.home-btn:hover {
  background: linear-gradient(135deg, #4338ca 0%, #6d28d9 100%);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(79, 70, 229, 0.4);
}

.home-icon {
  font-size: 1rem;
}

.home-text {
  font-size: 0.85rem;
}

.sidebar-header {
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.05);
  border-bottom: 1px solid #334155;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.sidebar-header h2 {
  font-size: 0.9rem;
  font-weight: 600;
}

.window-count {
  background: #4f46e5;
  color: white;
  padding: 0.15rem 0.5rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 600;
}

.window-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.window-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  margin-bottom: 0.25rem;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.window-item:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: #4f46e5;
}

.window-item.minimized {
  opacity: 0.6;
  background: rgba(255, 255, 255, 0.02);
}

.window-icon {
  font-size: 1rem;
}

.window-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.window-title {
  font-size: 0.75rem;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.window-status {
  font-size: 0.65rem;
  color: #94a3b8;
}

.window-close {
  background: transparent;
  border: none;
  color: #94a3b8;
  font-size: 1.1rem;
  cursor: pointer;
  padding: 0.15rem;
  line-height: 1;
  border-radius: 3px;
  transition: all 0.2s ease;
}

.window-close:hover {
  background: #dc3545;
  color: white;
}

.no-windows {
  text-align: center;
  padding: 1rem;
  color: #64748b;
  font-size: 0.8rem;
  font-style: italic;
}
</style>
