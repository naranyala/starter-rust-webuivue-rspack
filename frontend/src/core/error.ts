export class AppError extends Error {
  constructor(
    public code: string,
    message: string,
    public cause?: unknown
  ) {
    super(message);
    this.name = 'AppError';
  }

  static notFound(resource: string): AppError {
    return new AppError('NOT_FOUND', `Resource not found: ${resource}`);
  }

  static invalidInput(message: string): AppError {
    return new AppError('INVALID_INPUT', message);
  }

  static network(message: string, cause?: unknown): AppError {
    return new AppError('NETWORK_ERROR', message, cause);
  }

  static connection(message: string, cause?: unknown): AppError {
    return new AppError('CONNECTION_ERROR', message, cause);
  }

  static timeout(message: string): AppError {
    return new AppError('TIMEOUT', message);
  }

  static unknown(cause?: unknown): AppError {
    return new AppError('UNKNOWN', 'An unknown error occurred', cause);
  }
}

export type Result<T, E = AppError> = { ok: true; value: T } | { ok: false; error: E };

export function ok<T>(value: T): Result<T, never> {
  return { ok: true, value };
}

export function err<T>(error: AppError): Result<T, AppError> {
  return { ok: false, error };
}

export function isOk<T, E>(result: Result<T, E>): result is { ok: true; value: T } {
  return result.ok === true;
}

export function isErr<T, E>(result: Result<T, E>): result is { ok: false; error: E } {
  return result.ok === false;
}

export function map<T, U, E>(result: Result<T, E>, fn: (value: T) => U): Result<U, E> {
  if (isOk(result)) {
    return ok(fn(result.value));
  }
  return result as Result<U, E>;
}

export function mapErr<T, E, F>(result: Result<T, E>, fn: (error: E) => F): Result<T, F> {
  if (isOk(result)) {
    return result;
  }
  return { ok: false, error: fn(result.error) };
}

export async function tryCatch<T>(promise: Promise<T>): Promise<Result<T, AppError>> {
  try {
    const value = await promise;
    return ok(value);
  } catch (e) {
    if (e instanceof AppError) {
      return err(e);
    }
    return err(AppError.unknown(e));
  }
}
