<template>
  <div class="event-log">
    <div class="event-header">
      <h2>📡 Event Log</h2>
      <div class="header-actions">
        <Button variant="secondary" @click="refresh">Refresh</Button>
        <Button variant="danger" @click="clearHistory">Clear</Button>
      </div>
    </div>

    <div class="stats-bar">
      <div class="stat"><span class="stat-label">Total Events:</span> <span class="stat-value">{{ history.length }}</span></div>
      <div class="stat"><span class="stat-label">Event Types:</span> <span class="stat-value">{{ eventTypes.length }}</span></div>
      <div class="stat"><span class="stat-label">Listeners:</span> <span class="stat-value">{{ listenerCount }}</span></div>
    </div>

    <div class="event-list">
      <div v-if="history.length === 0" class="no-events">No events recorded yet</div>
      <div v-for="(event, index) in history" :key="index" class="event-item">
        <div class="event-time">{{ formatTime(event.timestamp) }}</div>
        <div class="event-source" :class="event.source">{{ event.source }}</div>
        <div class="event-name">{{ event.event }}</div>
        <div class="event-data">{{ JSON.stringify(event.data).substring(0, 100) }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import Button from '../../components/common/Button.vue';
import { useEventLogViewModel } from '../../viewmodels/useEventLogViewModel.ts';

const { history, eventTypes, listenerCount, refresh, clearHistory } = useEventLogViewModel();

const formatTime = (timestamp) => {
  return new Date(timestamp).toLocaleTimeString();
};
</script>

<style scoped>
.event-log { padding: 1rem; height: 100%; display: flex; flex-direction: column; }
.event-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }
.header-actions { display: flex; gap: 0.5rem; }
.stats-bar { display: flex; gap: 2rem; margin-bottom: 1rem; padding: 1rem; background: white; border-radius: 0.5rem; }
.stat { display: flex; gap: 0.5rem; }
.stat-label { color: #6b7280; }
.stat-value { font-weight: 600; color: #4f46e5; }
.event-list { flex: 1; overflow-y: auto; }
.no-events { text-align: center; color: #6b7280; padding: 2rem; }
.event-item { display: flex; gap: 1rem; padding: 0.75rem; background: white; border-radius: 0.5rem; margin-bottom: 0.5rem; align-items: center; }
.event-time { font-size: 0.75rem; color: #6b7280; min-width: 80px; }
.event-source { font-size: 0.7rem; padding: 0.25rem 0.5rem; border-radius: 9999px; text-transform: uppercase; font-weight: 600; }
.event-source.frontend { background: #dbeafe; color: #2563eb; }
.event-source.backend { background: #dcfce7; color: #16a34a; }
.event-name { font-weight: 600; min-width: 150px; }
.event-data { font-size: 0.75rem; color: #6b7280; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
