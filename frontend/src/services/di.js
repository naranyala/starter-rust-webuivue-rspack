// lib/di.js
// Dependency Injection Container for JavaScript/TypeScript frontend

class Container {
  constructor() {
    this.services = new Map();
    this.singletons = new Map();
  }

  /**
   * Register a service with the container
   * @param {string} name - The service name/identifier
   * @param {Function|Object} definition - The service definition (class constructor or factory function)
   * @param {Object} options - Registration options
   * @param {boolean} options.singleton - Whether to treat as singleton (default: true)
   * @param {Array} options.dependencies - Array of dependency names to inject
   */
  register(name, definition, options = {}) {
    const config = {
      singleton: true,
      dependencies: [],
      ...options,
    };

    this.services.set(name, {
      definition,
      ...config,
    });

    return this; // Enable chaining
  }

  /**
   * Register an existing instance (useful for singletons)
   * @param {string} name - The service name
   * @param {Object} instance - The existing instance
   */
  registerInstance(name, instance) {
    this.singletons.set(name, instance);
    return this;
  }

  /**
   * Resolve a service from the container
   * @param {string} name - The service name to resolve
   * @returns {Object} The resolved service instance
   */
  resolve(name) {
    // Return existing singleton if available
    if (this.singletons.has(name)) {
      return this.singletons.get(name);
    }

    const service = this.services.get(name);
    if (!service) {
      throw new Error(`Service '${name}' not found in container`);
    }

    // Resolve dependencies first
    const dependencies = service.dependencies.map((depName) => this.resolve(depName));

    // Create instance
    let instance;
    if (typeof service.definition === 'function') {
      // If it's a class constructor
      if (service.definition.prototype && service.definition.prototype.constructor) {
        instance = new service.definition(...dependencies);
      } else {
        // If it's a factory function
        instance = service.definition(...dependencies);
      }
    } else {
      // If it's a plain object/value
      instance = service.definition;
    }

    // Store as singleton if configured
    if (service.singleton) {
      this.singletons.set(name, instance);
    }

    return instance;
  }

  /**
   * Check if a service is registered
   * @param {string} name - The service name
   * @returns {boolean}
   */
  has(name) {
    return this.services.has(name) || this.singletons.has(name);
  }

  /**
   * Clear all registered services (useful for testing)
   */
  clear() {
    this.services.clear();
    this.singletons.clear();
  }

  /**
   * Create a child container that inherits from this one
   * @returns {Container}
   */
  createChild() {
    const child = new Container();
    child.parent = this;
    return child;
  }
}

// Global container instance
const globalContainer = new Container();

/**
 * Get the global container instance
 * @returns {Container}
 */
export function container() {
  return globalContainer;
}

/**
 * Decorator/Helper for injecting dependencies into a class
 * @param {Array<string>} dependencies - Array of service names to inject
 * @returns {Function}
 */
export function inject(dependencies) {
  return (target) => {
    target._injectDependencies = dependencies;
    return target;
  };
}

/**
 * Helper to register and resolve in one call
 * @param {string} name - Service name
 * @param {Function} definition - Service definition
 * @param {Object} options - Registration options
 * @returns {Object} The resolved instance
 */
export function singleton(name, definition, options = {}) {
  container().register(name, definition, { ...options, singleton: true });
  return container().resolve(name);
}

/**
 * Helper to register a transient service (non-singleton)
 * @param {string} name - Service name
 * @param {Function} definition - Service definition
 * @param {Object} options - Registration options
 */
export function transient(name, definition, options = {}) {
  container().register(name, definition, { ...options, singleton: false });
}

export { Container };
export default globalContainer;
