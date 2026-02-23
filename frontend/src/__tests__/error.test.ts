import { describe, expect, test } from 'bun:test';
import { AppError, err, isErr, isOk, map, mapErr, ok, tryCatch } from '../core/error';

describe('AppError', () => {
  test('creates error with code and message', () => {
    const error = new AppError('TEST', 'test message');
    expect(error.code).toBe('TEST');
    expect(error.message).toBe('test message');
    expect(error.name).toBe('AppError');
  });

  test('creates notFound error', () => {
    const error = AppError.notFound('user');
    expect(error.code).toBe('NOT_FOUND');
    expect(error.message).toBe('Resource not found: user');
  });

  test('creates invalidInput error', () => {
    const error = AppError.invalidInput('Invalid email');
    expect(error.code).toBe('INVALID_INPUT');
    expect(error.message).toBe('Invalid email');
  });

  test('creates network error with cause', () => {
    const cause = new Error('network down');
    const error = AppError.network('Connection failed', cause);
    expect(error.code).toBe('NETWORK_ERROR');
    expect(error.cause).toBe(cause);
  });

  test('creates connection error', () => {
    const error = AppError.connection('WebSocket closed');
    expect(error.code).toBe('CONNECTION_ERROR');
  });

  test('creates timeout error', () => {
    const error = AppError.timeout('Request timed out');
    expect(error.code).toBe('TIMEOUT');
  });

  test('creates unknown error', () => {
    const error = AppError.unknown();
    expect(error.code).toBe('UNKNOWN');
  });
});

describe('Result helpers', () => {
  test('ok creates success result', () => {
    const result = ok(42);
    expect(result.ok).toBe(true);
    expect(result.value).toBe(42);
  });

  test('err creates error result', () => {
    const error = AppError.notFound('item');
    const result = err(error);
    expect(result.ok).toBe(false);
    expect(result.error).toBe(error);
  });

  test('isOk returns true for success result', () => {
    const result = ok(42);
    expect(isOk(result)).toBe(true);
    expect(isErr(result)).toBe(false);
  });

  test('isErr returns true for error result', () => {
    const result = err(AppError.notFound('item'));
    expect(isErr(result)).toBe(true);
    expect(isOk(result)).toBe(false);
  });

  test('isOk type guard works', () => {
    const result: { ok: true; value: number } | { ok: false; error: AppError } = ok(42);
    if (isOk(result)) {
      expect(result.value * 2).toBe(84);
    }
  });
});

describe('map', () => {
  test('maps success result', () => {
    const result = ok(10);
    const mapped = map(result, (v) => v * 2);
    expect(isOk(mapped)).toBe(true);
    if (isOk(mapped)) {
      expect(mapped.value).toBe(20);
    }
  });

  test('passes through error result', () => {
    const error = AppError.notFound('item');
    const result = err(error);
    const mapped = map(result, (v: number) => v * 2);
    expect(isErr(mapped)).toBe(true);
  });
});

describe('mapErr', () => {
  test('maps error result', () => {
    const error = AppError.notFound('item');
    const result = err(error);
    const mapped = mapErr(result, (e) => {
      e.code = 'MAPPED';
      return e;
    });
    expect(isErr(mapped)).toBe(true);
  });

  test('passes through success result', () => {
    const result = ok(42);
    const mapped = mapErr(result, (e) => e);
    expect(isOk(mapped)).toBe(true);
  });
});

describe('tryCatch', () => {
  test('resolves promise successfully', async () => {
    const promise = Promise.resolve(42);
    const result = await tryCatch(promise);
    expect(isOk(result)).toBe(true);
    if (isOk(result)) {
      expect(result.value).toBe(42);
    }
  });

  test('catches rejected promise as AppError', async () => {
    const promise = Promise.reject(new Error('failed'));
    const result = await tryCatch(promise);
    expect(isErr(result)).toBe(true);
    if (isErr(result)) {
      expect(result.error.code).toBe('UNKNOWN');
    }
  });

  test('catches AppError as-is', async () => {
    const error = AppError.notFound('user');
    const promise = Promise.reject(error);
    const result = await tryCatch(promise);
    expect(isErr(result)).toBe(true);
    if (isErr(result)) {
      expect(result.error.code).toBe('NOT_FOUND');
    }
  });
});
