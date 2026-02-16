// frontend/src/services/logger.js
// Enhanced logging service for the frontend

class Logger {
  constructor() {
    this.logs = [];
    this.maxLogEntries = 1000; // Limit log entries to prevent memory issues
    this.logLevel = 'INFO'; // Default log level
    this.logLevels = {
      DEBUG: 0,
      INFO: 1,
      WARN: 2,
      ERROR: 3,
    };
  }

  // Check if a log level should be logged based on current level
  shouldLog(level) {
    return this.logLevels[level] >= this.logLevels[this.logLevel];
  }

  // Add a log entry
  addLog(level, message, meta = {}) {
    if (!this.shouldLog(level)) return;

    const timestamp = new Date().toISOString();
    const logEntry = {
      timestamp,
      level,
      message,
      meta,
      id: Date.now() + Math.random(), // Unique ID for the log entry
    };

    this.logs.push(logEntry);

    // Maintain max log count
    if (this.logs.length > this.maxLogEntries) {
      this.logs = this.logs.slice(-this.maxLogEntries);
    }

    // Output to console
    this.outputToConsole(logEntry);

    // Emit event for Vue components to react to
    this.emitLogEvent(logEntry);
  }

  // Output log to console with proper formatting
  outputToConsole(entry) {
    const { level, message, timestamp } = entry;
    const formattedMessage = `[${timestamp}] ${level} - ${message}`;

    switch (level) {
      case 'ERROR':
        console.error(formattedMessage);
        break;
      case 'WARN':
        console.warn(formattedMessage);
        break;
      case 'INFO':
        console.info(formattedMessage);
        break;
      case 'DEBUG':
        console.debug(formattedMessage);
        break;
      default:
        console.log(formattedMessage);
    }
  }

  // Emit custom event for Vue components
  emitLogEvent(entry) {
    const event = new CustomEvent('logEntryAdded', { detail: entry });
    window.dispatchEvent(event);
  }

  // Log methods
  debug(message, meta = {}) {
    this.addLog('DEBUG', message, meta);
  }

  info(message, meta = {}) {
    this.addLog('INFO', message, meta);
  }

  warn(message, meta = {}) {
    this.addLog('WARN', message, meta);
  }

  error(message, meta = {}) {
    this.addLog('ERROR', message, meta);
  }

  // Get all logs
  getLogs() {
    return [...this.logs]; // Return a copy
  }

  // Clear logs
  clearLogs() {
    this.logs = [];
    window.dispatchEvent(new CustomEvent('logsCleared'));
  }

  // Set log level
  setLogLevel(level) {
    if (Object.hasOwn(this.logLevels, level.toUpperCase())) {
      this.logLevel = level.toUpperCase();
      this.info(`Log level set to ${this.logLevel}`);
    } else {
      console.warn(`Invalid log level: ${level}. Valid levels: DEBUG, INFO, WARN, ERROR`);
    }
  }

  // Get current log level
  getLogLevel() {
    return this.logLevel;
  }
}

// Create a singleton instance
const logger = new Logger();

// Make it globally available
window.Logger = logger;

export default logger;
