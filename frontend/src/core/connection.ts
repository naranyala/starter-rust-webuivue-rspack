export type ConnectionStatus =
  | 'initializing'
  | 'connecting'
  | 'connected'
  | 'disconnected'
  | 'error';

export interface ConnectionStats {
  latency: number | null;
  messagesSent: number;
  messagesReceived: number;
  messagesQueued: number;
  lastConnected: Date | null;
  lastError: string | null;
}

export interface ConnectionListener {
  onStatusChange: (status: ConnectionStatus, stats: ConnectionStats) => void;
  onMessageReceived: (data: unknown) => void;
}

class ConnectionManager {
  private status: ConnectionStatus = 'initializing';
  private stats: ConnectionStats = {
    latency: null,
    messagesSent: 0,
    messagesReceived: 0,
    messagesQueued: 0,
    lastConnected: null,
    lastError: null,
  };
  private listeners: Set<ConnectionListener> = new Set();
  private checkInterval: ReturnType<typeof setInterval> | null = null;

  constructor() {
    this.startMonitoring();
  }

  private startMonitoring() {
    this.checkInterval = setInterval(() => {
      const isAvailable = this.isWebUIAvailable();
      const previousStatus = this.status;

      if (isAvailable && this.status !== 'connected') {
        this.setStatus('connected');
      } else if (!isAvailable && this.status === 'connected') {
        this.setStatus('disconnected');
      }

      if (previousStatus !== this.status) {
        this.notifyListeners();
      }
    }, 1000);
  }

  private isWebUIAvailable(): boolean {
    try {
      const webui = (window as any).webui;
      return !!webui && typeof webui.call === 'function';
    } catch {
      return false;
    }
  }

  private setStatus(newStatus: ConnectionStatus) {
    this.status = newStatus;
    if (newStatus === 'connected') {
      this.stats.lastConnected = new Date();
    } else if (newStatus === 'error') {
      this.stats.lastError = 'Connection error';
    }
  }

  private notifyListeners() {
    for (const listener of this.listeners) {
      try {
        listener.onStatusChange(this.status, { ...this.stats });
      } catch (e) {
        console.error('Error in connection listener:', e);
      }
    }
  }

  subscribe(listener: ConnectionListener): () => void {
    this.listeners.add(listener);
    listener.onStatusChange(this.status, { ...this.stats });
    return () => this.listeners.delete(listener);
  }

  getStatus(): ConnectionStatus {
    return this.status;
  }

  getStats(): ConnectionStats {
    return { ...this.stats };
  }

  isConnected(): boolean {
    return this.status === 'connected';
  }

  dispose() {
    if (this.checkInterval) {
      clearInterval(this.checkInterval);
    }
    this.listeners.clear();
  }
}

export const connectionManager = new ConnectionManager();
