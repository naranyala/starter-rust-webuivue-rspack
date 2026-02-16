import { ref } from 'vue';
import type { EventData } from '../../types';

type EventCallback = (data: unknown) => void;

class EventBus {
  private listeners: Map<string, Set<EventCallback>> = new Map();
  private backendListeners: EventCallback[] = [];
  private eventHistory: EventData[] = [];
  private maxHistory = 100;

  on(event: string, callback: EventCallback): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(callback);
    return () => this.off(event, callback);
  }

  once(event: string, callback: EventCallback): () => void {
    const wrapper: EventCallback = (...args) => {
      callback(args[0]);
      this.off(event, wrapper);
    };
    return this.on(event, wrapper);
  }

  off(event: string, callback: EventCallback): void {
    if (this.listeners.has(event)) {
      this.listeners.get(event)!.delete(callback);
    }
  }

  emit(event: string, data: unknown = null): void {
    const timestamp = new Date().toISOString();
    const eventData: EventData = { event, data, timestamp, source: 'frontend' };

    this.addToHistory(eventData);

    if (this.listeners.has(event)) {
      this.listeners.get(event)!.forEach((callback) => {
        try {
          callback(data);
        } catch (e) {
          console.error(`Error in event handler for "${event}":`, e);
        }
      });
    }

    this.listeners.get('*')?.forEach((callback) => {
      try {
        callback(eventData);
      } catch (e) {
        console.error(`Error in wildcard event handler:`, e);
      }
    });
  }

  onBackendEvent(callback: EventCallback): () => void {
    this.backendListeners.push(callback);
    return () => {
      this.backendListeners = this.backendListeners.filter((cb) => cb !== callback);
    };
  }

  handleBackendEvent(envelope: {
    event: { type: string; data?: unknown };
    timestamp: string;
    source: string;
    id?: string;
  }): void {
    const { event, timestamp, source } = envelope;
    const eventData: EventData = {
      event: event.type,
      data: event.data || event,
      timestamp,
      source: source || 'backend',
      id: envelope.id,
    };

    this.addToHistory(eventData);

    if (this.listeners.has(event.type)) {
      this.listeners.get(event.type)!.forEach((callback) => {
        try {
          callback(event.data || event);
        } catch (e) {
          console.error(`Error in backend event handler for "${event.type}":`, e);
        }
      });
    }

    this.backendListeners.forEach((callback) => {
      try {
        callback(eventData);
      } catch (e) {
        console.error(`Error in backend event listener:`, e);
      }
    });

    this.listeners.get('*')?.forEach((callback) => {
      try {
        callback(eventData);
      } catch (e) {
        console.error(`Error in wildcard event handler:`, e);
      }
    });
  }

  private addToHistory(eventData: EventData): void {
    this.eventHistory.unshift(eventData);
    if (this.eventHistory.length > this.maxHistory) {
      this.eventHistory = this.eventHistory.slice(0, this.maxHistory);
    }
  }

  getHistory(): EventData[] {
    return [...this.eventHistory];
  }

  clearHistory(): void {
    this.eventHistory = [];
  }

  listenerCount(event?: string): number {
    if (event) {
      return this.listeners.get(event)?.size || 0;
    }
    let total = 0;
    for (const set of this.listeners.values()) {
      total += set.size;
    }
    return total;
  }

  eventTypes(): string[] {
    return Array.from(this.listeners.keys());
  }
}

export const eventBus = new EventBus();

if (typeof window !== 'undefined') {
  (window as unknown as { __eventBus: EventBus }).__eventBus = eventBus;
}

export function useEventBus() {
  const on = (event: string, callback: EventCallback) => eventBus.on(event, callback);
  const once = (event: string, callback: EventCallback) => eventBus.once(event, callback);
  const off = (event: string, callback: EventCallback) => eventBus.off(event, callback);
  const emit = (event: string, data: unknown) => eventBus.emit(event, data);

  const onBackendEvent = (callback: EventCallback) => eventBus.onBackendEvent(callback);
  const handleBackendEvent = (envelope: {
    event: { type: string; data?: unknown };
    timestamp: string;
    source: string;
    id?: string;
  }) => eventBus.handleBackendEvent(envelope);

  const history = ref(eventBus.getHistory());
  const eventTypes = ref(eventBus.eventTypes());
  const listenerCount = ref(eventBus.listenerCount());

  const refreshStats = () => {
    history.value = eventBus.getHistory();
    eventTypes.value = eventBus.eventTypes();
    listenerCount.value = eventBus.listenerCount();
  };

  return {
    on,
    once,
    off,
    emit,
    onBackendEvent,
    handleBackendEvent,
    history,
    eventTypes,
    listenerCount,
    refreshStats,
    eventBus,
  };
}

export function createEventBus(): EventBus {
  return new EventBus();
}

export default eventBus;
