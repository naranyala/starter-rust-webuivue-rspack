import { onMounted, onUnmounted, ref } from 'vue';
import { useSystemStore } from '../models/stores';

export function useSystemInfoViewModel() {
  const systemStore = useSystemStore();
  const currentTime = ref(new Date().toLocaleString());
  let timeInterval: ReturnType<typeof setInterval> | null = null;

  const refreshInfo = async () => {
    await systemStore.fetchSystemInfo();
    await systemStore.fetchUptime();
  };

  const formatUptime = (seconds: number): string => {
    if (!seconds) return 'N/A';
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    let result = '';
    if (days > 0) result += `${days}d `;
    if (hours > 0) result += `${hours}h `;
    if (minutes > 0) result += `${minutes}m`;
    return result.trim() || 'Less than 1 minute';
  };

  const formatTimestamp = (timestamp?: number): string => {
    if (!timestamp) return 'N/A';
    return new Date(timestamp * 1000).toLocaleString();
  };

  onMounted(async () => {
    await refreshInfo();
    timeInterval = setInterval(() => {
      currentTime.value = new Date().toLocaleString();
    }, 1000);
  });

  onUnmounted(() => {
    if (timeInterval) clearInterval(timeInterval);
  });

  return {
    systemStore,
    currentTime,
    refreshInfo,
    formatUptime,
    formatTimestamp,
  };
}
