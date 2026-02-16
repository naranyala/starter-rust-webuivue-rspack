// Simple window tracker

import { getConnectionStatus, sendToBackend } from '../webuiConnection.ts';

const log = (...args) => console.log('[Tracker]', ...args);

class WindowTracker {
  constructor() {
    this.states = new Map();
    this.enabled = true;
    log('Initialized');
  }

  enable() {
    this.enabled = true;
  }
  disable() {
    this.enabled = false;
  }

  async track(id, title, component, newState) {
    if (!this.enabled) return;

    const prev = this.states.get(id) || null;
    this.states.set(id, newState);

    const payload = {
      window_id: id,
      window_title: title,
      component: component,
      previous_state: prev,
      new_state: newState,
      timestamp: new Date().toISOString(),
    };

    log('Track:', newState, title);

    // Send to backend - will get JSON response
    try {
      const jsonPayload = JSON.stringify(payload);
      const result = await sendToBackend('window_state_change', jsonPayload);
      log('Result:', result);
    } catch (e) {
      log('Error:', e.message);
    }
  }

  async trackWindowOpened(id, title, comp) {
    await this.track(id, title, comp, 'opened');
  }
  async trackFocused(id, title, comp) {
    await this.track(id, title, comp, 'focused');
  }
  async trackMinimized(id, title, comp) {
    await this.track(id, title, comp, 'minimized');
  }
  async trackRestored(id, title, comp) {
    await this.track(id, title, comp, 'restored');
  }
  async trackMaximized(id, title, comp) {
    await this.track(id, title, comp, 'maximized');
  }
  async trackClosed(id, title, comp) {
    await this.track(id, title, comp, 'closed');
    this.states.delete(id);
  }

  getState(id) {
    return this.states.get(id);
  }
  getAllStates() {
    return new Map(this.states);
  }
  isConnected() {
    return getConnectionStatus().connected;
  }

  // Additional methods to match the full interface
  getWindowState(windowId) {
    return this.getState(windowId);
  }
  getAllWindowStates() {
    return this.getAllStates();
  }
  isBackendConnected() {
    return this.isConnected();
  }
  getPendingCount() {
    return 0;
  } // Simple tracker doesn't track pending requests
}

export const windowTracker = new WindowTracker();

export function useWindowTracker() {
  return {
    trackOpened: (...args) => windowTracker.trackWindowOpened(...args),
    trackFocused: (...args) => windowTracker.trackFocused(...args),
    trackMinimized: (...args) => windowTracker.trackMinimized(...args),
    trackRestored: (...args) => windowTracker.trackRestored(...args),
    trackMaximized: (...args) => windowTracker.trackMaximized(...args),
    trackClosed: (...args) => windowTracker.trackClosed(...args),
    enable: () => windowTracker.enable(),
    disable: () => windowTracker.disable(),
    getState: (...args) => windowTracker.getState(...args),
    getAllStates: () => windowTracker.getAllStates(),
    isConnected: () => windowTracker.isConnected(),
  };
}
