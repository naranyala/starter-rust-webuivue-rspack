import type { WindowStateChangePayload } from '../types';

const DEBUG = true;
function log(...args: unknown[]) {
  if (DEBUG) {
    console.log('[ConnectionManager]', ...args);
  }
}

export type ConnectionState =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'reconnecting'
  | 'error';

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
}

interface ConnectionListener {
  onStateChange: (state: ConnectionState, info: ConnectionStateInfo) => void;
  onError: (error: Error, info: ConnectionStateInfo) => void;
  onMessageSent: (payload: WindowStateChangePayload) => void;
  onMessageReceived: (data: unknown) => void;
}

export class ConnectionManager {
  private static instance: ConnectionManager;
  private state: ConnectionState = 'disconnected';
  private stateInfo: ConnectionStateInfo = {
    state: 'disconnected',
    lastConnectedAt: null,
    lastDisconnectedAt: null,
    lastErrorAt: null,
    lastError: null,
    reconnectAttempts: 0,
    totalMessagesSent: 0,
    totalMessagesReceived: 0,
    pendingMessages: 0,
  };
  private listeners: Set<ConnectionListener> = new Set();
  private checkInterval: ReturnType<typeof setInterval> | null = null;
  private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
  private messageQueue: WindowStateChangePayload[] = [];
  private readonly RECONNECT_DELAY = 2000;
  private readonly MAX_RECONNECT_ATTEMPTS = 10;

  private constructor() {
    this.startMonitoring();
    this.setupErrorHandlers();
  }

  static getInstance(): ConnectionManager {
    if (!ConnectionManager.instance) {
      ConnectionManager.instance = new ConnectionManager();
    }
    return ConnectionManager.instance;
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

      if (isAvailable && this.state !== 'connected') {
        this.transitionTo('connected');
        this.flushQueue();
      } else if (!isAvailable && this.state === 'connected') {
        this.transitionTo('disconnected');
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

  private transitionTo(newState: ConnectionState) {
    const oldState = this.state;
    this.state = newState;
    this.stateInfo.state = newState;

    log(`State transition: ${oldState} -> ${newState}`);

    switch (newState) {
      case 'connected':
        this.stateInfo.lastConnectedAt = new Date();
        this.stateInfo.reconnectAttempts = 0;
        break;
      case 'disconnected':
        this.stateInfo.lastDisconnectedAt = new Date();
        break;
      case 'error':
        this.stateInfo.lastErrorAt = new Date();
        break;
      case 'reconnecting':
        this.stateInfo.reconnectAttempts++;
        break;
    }

    this.notifyStateChange();
  }

  private handleError(error: Error) {
    log('Error occurred:', error.message);
    this.stateInfo.lastError = error.message;
    this.stateInfo.lastErrorAt = new Date();

    this.transitionTo('error');

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

    this.transitionTo('reconnecting');

    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
    }

    this.reconnectTimeout = setTimeout(() => {
      if (this.isWebUIAvailable()) {
        this.transitionTo('connected');
        this.flushQueue();
      } else {
        this.transitionTo('disconnected');
        if (this.stateInfo.reconnectAttempts < this.MAX_RECONNECT_ATTEMPTS) {
          this.attemptReconnect();
        }
      }
    }, this.RECONNECT_DELAY);
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
      if (this.state === 'connected') {
        try {
          const webui = (window as any).webui;
          if (!webui) {
            this.handleError(new Error('WebUI object disappeared'));
            return;
          }
          // Try a simple call to verify it's working
          await webui.call('ping_backend');
        } catch (error) {
          this.handleError(error instanceof Error ? error : new Error('WebUI health check failed'));
        }
      }
    }, 5000);
  }

  private async flushQueue() {
    if (this.messageQueue.length === 0) return;

    log(`Flushing ${this.messageQueue.length} queued messages`);

    const messages = [...this.messageQueue];
    this.messageQueue = [];

    for (const payload of messages) {
      await this.send(payload, false);
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
  subscribe(listener: ConnectionListener): () => void {
    this.listeners.add(listener);
    // Immediately notify with current state
    listener.onStateChange(this.state, { ...this.stateInfo });

    return () => {
      this.listeners.delete(listener);
    };
  }

  async send(
    payload: WindowStateChangePayload,
    queueIfDisconnected: boolean = true
  ): Promise<boolean> {
    try {
      if (this.state !== 'connected') {
        if (queueIfDisconnected) {
          log('Queueing message (not connected)');
          this.messageQueue.push(payload);
          if (this.messageQueue.length > 100) {
            this.messageQueue.shift();
          }
          this.stateInfo.pendingMessages = this.messageQueue.length;
        }
        return false;
      }

      const webui = (window as any).webui;
      const jsonStr = JSON.stringify(payload);
      const elementName = `window_state_changed:${jsonStr}`;

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

  getState(): ConnectionState {
    return this.state;
  }

  getStateInfo(): ConnectionStateInfo {
    return { ...this.stateInfo };
  }

  isConnected(): boolean {
    return this.state === 'connected';
  }

  forceReconnect() {
    this.transitionTo('disconnected');
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

export const connectionManager = ConnectionManager.getInstance();
