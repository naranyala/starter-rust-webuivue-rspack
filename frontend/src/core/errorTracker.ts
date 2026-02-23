import { ref } from 'vue';

export interface ErrorLog {
  id: string;
  timestamp: string;
  level: 'error' | 'warn' | 'info';
  context: string;
  message: string;
  stack?: string;
  extra?: Record<string, unknown>;
  userAgent?: string;
  url?: string;
}

const errorLogs = ref<ErrorLog[]>([]);
const maxLogs = 100;

function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

function getUserAgent(): string {
  return typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown';
}

function getUrl(): string {
  return typeof window !== 'undefined' ? window.location.href : 'unknown';
}

export function logError(
  context: string,
  error: unknown,
  extra?: Record<string, unknown>
): ErrorLog {
  const timestamp = new Date().toISOString();
  let message = 'Unknown error';
  let stack: string | undefined;

  if (error instanceof Error) {
    message = error.message;
    stack = error.stack;
  } else if (typeof error === 'string') {
    message = error;
  } else if (error && typeof error === 'object') {
    message = ((error as Record<string, unknown>).message as string) ?? JSON.stringify(error);
  }

  const logEntry: ErrorLog = {
    id: generateId(),
    timestamp,
    level: 'error',
    context,
    message,
    stack,
    extra,
    userAgent: getUserAgent(),
    url: getUrl(),
  };

  errorLogs.value.unshift(logEntry);

  if (errorLogs.value.length > maxLogs) {
    errorLogs.value = errorLogs.value.slice(0, maxLogs);
  }

  const logger = (window as unknown as { logger?: { error: (...args: unknown[]) => void } }).logger;
  if (logger?.error) {
    logger.error(`[${timestamp}] [${context}] ${message}`, {
      ...extra,
      stack,
      url: logEntry.url,
    });
  } else {
    console.error(`[${timestamp}] [${context}] ${message}`, {
      ...extra,
      stack,
      url: logEntry.url,
    });
  }

  return logEntry;
}

export function logWarn(
  context: string,
  message: string,
  extra?: Record<string, unknown>
): ErrorLog {
  const timestamp = new Date().toISOString();

  const logEntry: ErrorLog = {
    id: generateId(),
    timestamp,
    level: 'warn',
    context,
    message,
    extra,
    userAgent: getUserAgent(),
    url: getUrl(),
  };

  errorLogs.value.unshift(logEntry);

  if (errorLogs.value.length > maxLogs) {
    errorLogs.value = errorLogs.value.slice(0, maxLogs);
  }

  const logger = (window as unknown as { logger?: { warn: (...args: unknown[]) => void } }).logger;
  if (logger?.warn) {
    logger.warn(`[${timestamp}] [${context}] ${message}`, extra);
  } else {
    console.warn(`[${timestamp}] [${context}] ${message}`, extra);
  }

  return logEntry;
}

export function logInfo(context: string, message: string, extra?: Record<string, unknown>): void {
  const timestamp = new Date().toISOString();
  const logger = (window as unknown as { logger?: { info: (...args: unknown[]) => void } }).logger;

  if (logger?.info) {
    logger.info(`[${timestamp}] [${context}] ${message}`, extra);
  } else {
    console.info(`[${timestamp}] [${context}] ${message}`, extra);
  }
}

export function getErrorLogs(): ErrorLog[] {
  return errorLogs.value;
}

export function clearErrorLogs(): void {
  errorLogs.value = [];
}

export function getErrorLogsByLevel(level: ErrorLog['level']): ErrorLog[] {
  return errorLogs.value.filter((log) => log.level === level);
}

export function useErrorTracker() {
  return {
    logError,
    logWarn,
    logInfo,
    getErrorLogs,
    clearErrorLogs,
    getErrorLogsByLevel,
    errorLogs,
  };
}
