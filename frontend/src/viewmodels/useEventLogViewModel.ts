import { useEventBus } from '../models/services';

export function useEventLogViewModel() {
  const { history, eventTypes, listenerCount, refreshStats, clearHistory } = useEventBus();

  const refresh = () => {
    refreshStats();
  };

  return {
    history,
    eventTypes,
    listenerCount,
    refresh,
    clearHistory,
  };
}
