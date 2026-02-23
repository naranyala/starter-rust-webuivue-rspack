import type { ConnectionStateEvent } from '../types';

const DEBUG = true;
function log(...args: unknown[]) {
  if (DEBUG) {
    console.log('[ConnectionStateTracker]', ...args);
  }
}

export enum ConnectionState {
  DISCONNECTED = 'disconnected',
  CONNECTING = 'connecting',
  CONNECTED = 'connected',
  RECONNECTING = 'reconnecting',
  ERROR = 'error',
}

export interface ConnectionStateInfo {
  state: ConnectionState;
  lastConnectedAt: Date | null;
  lastDisconnectedAt: Date | null;
  lastErrorAt: Date | null;
  lastError: string | null;
  reconnectAttempts: number;
  totalMessagesSent: number;
  totalMessagesReceived: number;
  pendingMessages: number;
  latency: number | null;
  port: string | null;
}

export interface ConnectionStateListener {
  onStateChange: (state: ConnectionState, info: ConnectionStateInfo) => void;
  onError: (error: Error, info: ConnectionStateInfo) => void;
  onMessageSent: (payload: any) => void;
  onMessageReceived: (data: any) => void;
}

export class ConnectionStateTracker {
  private static instance: ConnectionStateTracker;
  private state: ConnectionState = ConnectionState.DISCONNECTED;
  private stateInfo: ConnectionStateInfo;
  private listeners: Set<ConnectionStateListener> = new Set();
  private checkInterval: ReturnType<typeof setInterval> | null = null;
  private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
  private messageQueue: any[] = [];
  private readonly RECONNECT_DELAY = 2000;
  private readonly MAX_RECONNECT_ATTEMPTS = 10;
  private readonly CONNECTION_TIMEOUT = 5000;

  private constructor() {
    this.stateInfo = {
      state: ConnectionState.DISCONNECTED,
      lastConnectedAt: null,
      lastDisconnectedAt: null,
      lastErrorAt: null,
      lastError: null,
      reconnectAttempts: 0,
      totalMessagesSent: 0,
      totalMessagesReceived: 0,
      pendingMessages: 0,
      latency: null,
      port: null,
    };

    this.startMonitoring();
    this.setupErrorHandlers();
  }

  static getInstance(): ConnectionStateTracker {
    if (!ConnectionStateTracker.instance) {
      ConnectionStateTracker.instance = new ConnectionStateTracker();
    }
    return ConnectionStateTracker.instance;
  }

  private setupErrorHandlers() {
    // Global error handlers
    window.addEventListener('error', (event) => {
      this.handleError(new Error(`Global error: ${event.message}`));
    });

    window.addEventListener('unhandledrejection', (event) => {
      const error =
        event.reason instanceof Error
          ? event.reason
          : new Error(`Unhandled rejection: ${event.reason}`);
      this.handleError(error);
    });

    // WebUI specific error detection
    this.checkWebUIHealth();
  }

  private startMonitoring() {
    const lastState = this.state;

    this.checkInterval = setInterval(() => {
      const isAvailable = this.isWebUIAvailable();
      const previousState = this.state;

      if (isAvailable && this.state !== ConnectionState.CONNECTED) {
        this.transitionTo(ConnectionState.CONNECTED);
        this.flushQueue();
      } else if (!isAvailable && this.state === ConnectionState.CONNECTED) {
        this.transitionTo(ConnectionState.DISCONNECTED);
        this.attemptReconnect();
      }

      // Update pending count
      this.stateInfo.pendingMessages = this.messageQueue.length;

      // Notify listeners if state changed
      if (previousState !== this.state) {
        this.notifyStateChange();
      }
    }, 500);
  }

  private isWebUIAvailable(): boolean {
    try {
      const webui = (window as any).webui;
      return !!webui && typeof webui.call === 'function';
    } catch {
      return false;
    }
  }

  private checkWebUIHealth() {
    // Periodically check if webui is still responsive
    setInterval(async () => {
      if (this.state === ConnectionState.CONNECTED) {
        try {
          // Test connection by sending a ping
          await this.testConnection();
        } catch (error) {
          this.handleError(error instanceof Error ? error : new Error('WebUI health check failed'));
        }
      }
    }, 5000);
  }

  private transitionTo(newState: ConnectionState) {
    const oldState = this.state;
    this.state = newState;
    this.stateInfo.state = newState;

    log(`State transition: ${oldState} -> ${newState}`);

    switch (newState) {
      case ConnectionState.CONNECTED:
        this.stateInfo.lastConnectedAt = new Date();
        this.stateInfo.reconnectAttempts = 0;
        break;
      case ConnectionState.DISCONNECTED:
        this.stateInfo.lastDisconnectedAt = new Date();
        break;
      case ConnectionState.ERROR:
        this.stateInfo.lastErrorAt = new Date();
        break;
      case ConnectionState.RECONNECTING:
        this.stateInfo.reconnectAttempts++;
        break;
    }

    this.notifyStateChange();
  }

  private handleError(error: Error) {
    log('Error occurred:', error.message);
    this.stateInfo.lastError = error.message;
    this.stateInfo.lastErrorAt = new Date();

    this.transitionTo(ConnectionState.ERROR);

    this.listeners.forEach((listener) => {
      try {
        listener.onError(error, { ...this.stateInfo });
      } catch (e) {
        console.error('Error in listener:', e);
      }
    });

    // Attempt reconnect after error
    setTimeout(() => this.attemptReconnect(), this.RECONNECT_DELAY);
  }

  private attemptReconnect() {
    if (this.stateInfo.reconnectAttempts >= this.MAX_RECONNECT_ATTEMPTS) {
      log('Max reconnect attempts reached');
      return;
    }

    this.transitionTo(ConnectionState.RECONNECTING);

    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
    }

    this.reconnectTimeout = setTimeout(() => {
      if (this.isWebUIAvailable()) {
        this.transitionTo(ConnectionState.CONNECTED);
        this.flushQueue();
      } else {
        this.transitionTo(ConnectionState.DISCONNECTED);
        if (this.stateInfo.reconnectAttempts < this.MAX_RECONNECT_ATTEMPTS) {
          this.attemptReconnect();
        }
      }
    }, this.RECONNECT_DELAY);
  }

  private async flushQueue() {
    if (this.messageQueue.length === 0) return;

    log(`Flushing ${this.messageQueue.length} queued messages`);

    const messages = [...this.messageQueue];
    this.messageQueue = [];

    for (const payload of messages) {
      try {
        await this.send(payload, false);
      } catch (error) {
        log('Error sending queued message:', error);
        // Put it back in queue if failed
        this.messageQueue.unshift(payload);
      }
    }
  }

  private notifyStateChange() {
    const info = { ...this.stateInfo };
    this.listeners.forEach((listener) => {
      try {
        listener.onStateChange(this.state, info);
      } catch (e) {
        console.error('Error in state change listener:', e);
      }
    });
  }

  // Public API
  subscribe(listener: ConnectionStateListener): () => void {
    this.listeners.add(listener);
    // Immediately notify with current state
    listener.onStateChange(this.state, { ...this.stateInfo });

    return () => {
      this.listeners.delete(listener);
    };
  }

  async send(payload: any, queueIfDisconnected: boolean = true): Promise<boolean> {
    try {
      if (this.state !== ConnectionState.CONNECTED) {
        if (queueIfDisconnected) {
          log('Queueing message (not connected)');
          this.messageQueue.push(payload);
          if (this.messageQueue.length > 100) {
            this.messageQueue.shift(); // Remove oldest if queue gets too big
          }
          this.stateInfo.pendingMessages = this.messageQueue.length;
        }
        return false;
      }

      const webui = (window as any).webui;
      const jsonStr = JSON.stringify(payload);
      const elementName = `connection_state_change:${jsonStr}`;

      await webui.call(elementName);

      this.stateInfo.totalMessagesSent++;

      this.listeners.forEach((listener) => {
        try {
          listener.onMessageSent(payload);
        } catch (e) {
          console.error('Error in message sent listener:', e);
        }
      });

      return true;
    } catch (error) {
      this.handleError(error instanceof Error ? error : new Error('Send failed'));
      return false;
    }
  }

  async testConnection(): Promise<boolean> {
    if (this.state !== ConnectionState.CONNECTED) {
      return false;
    }

    try {
      const startTime = Date.now();
      const webui = (window as any).webui;

      // Send a ping to test connection
      await webui.call('ping_backend');

      const endTime = Date.now();
      this.stateInfo.latency = endTime - startTime;

      return true;
    } catch (error) {
      this.handleError(error instanceof Error ? error : new Error('Ping failed'));
      return false;
    }
  }

  getState(): ConnectionState {
    return this.state;
  }

  getStateInfo(): ConnectionStateInfo {
    return { ...this.stateInfo };
  }

  isConnected(): boolean {
    return this.state === ConnectionState.CONNECTED;
  }

  getPendingCount(): number {
    return this.messageQueue.length;
  }

  forceReconnect() {
    this.transitionTo(ConnectionState.DISCONNECTED);
    this.stateInfo.reconnectAttempts = 0;
    this.attemptReconnect();
  }

  dispose() {
    if (this.checkInterval) {
      clearInterval(this.checkInterval);
    }
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
    }
    this.listeners.clear();
  }
}

export const connectionStateTracker = ConnectionStateTracker.getInstance();
