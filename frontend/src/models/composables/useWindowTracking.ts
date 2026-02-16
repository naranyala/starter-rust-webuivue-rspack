import { computed, onMounted, ref } from 'vue';
import { windowTracker } from '../services';

export interface WindowTrackingInfo {
  windowId: number;
  windowTitle: string;
  component: string;
  state: string;
  timestamp: string;
}

export function useWindowTracking() {
  const isTrackingEnabled = ref(true);
  const lastEvent = ref<WindowTrackingInfo | null>(null);
  const isBackendConnected = ref(false);

  // Check connection status periodically
  onMounted(() => {
    const checkConnection = () => {
      isBackendConnected.value = windowTracker.isBackendConnected();
    };
    checkConnection();
    setInterval(checkConnection, 1000);
  });

  const enableTracking = () => {
    windowTracker.enable();
    isTrackingEnabled.value = true;
  };

  const disableTracking = () => {
    windowTracker.disable();
    isTrackingEnabled.value = false;
  };

  const getWindowState = (windowId: number): string | undefined => {
    return windowTracker.getWindowState(windowId);
  };

  const getAllWindowStates = (): Map<number, string> => {
    return windowTracker.getAllWindowStates();
  };

  const windowStatesList = computed(() => {
    const states = getAllWindowStates();
    return Array.from(states.entries()).map(([id, state]) => ({
      windowId: id,
      state,
    }));
  });

  const pendingCount = computed(() => {
    return windowTracker.getPendingCount();
  });

  return {
    isTrackingEnabled,
    isBackendConnected,
    lastEvent,
    pendingCount,
    windowStatesList,
    enableTracking,
    disableTracking,
    getWindowState,
    getAllWindowStates,
    trackOpened: windowTracker.trackWindowOpened.bind(windowTracker),
    trackFocused: windowTracker.trackFocused.bind(windowTracker),
    trackMinimized: windowTracker.trackMinimized.bind(windowTracker),
    trackRestored: windowTracker.trackRestored.bind(windowTracker),
    trackMaximized: windowTracker.trackMaximized.bind(windowTracker),
    trackClosed: windowTracker.trackClosed.bind(windowTracker),
  };
}
