// infrastructure/di.rs
// Dependency Injection Container for Rust backend

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Type-erased container for storing services
pub struct Container {
    services: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl Container {
    /// Create a new empty container
    pub fn new() -> Self {
        Self {
            services: Mutex::new(HashMap::new()),
        }
    }

    /// Register a service instance
    pub fn register<T: 'static + Send + Sync>(&self, instance: T) {
        let type_id = TypeId::of::<T>();
        let mut services = self.services.lock().unwrap();
        services.insert(type_id, Arc::new(instance));
    }

    /// Resolve a service by type
    #[allow(dead_code)]
    pub fn resolve<T: 'static + Clone>(&self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let services = self.services.lock().unwrap();

        services
            .get(&type_id)
            .and_then(|service| service.downcast_ref::<T>().cloned())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static::lazy_static! {
    /// Global container instance
    static ref GLOBAL_CONTAINER: Container = Container::new();
}

/// Get the global container
pub fn container() -> &'static Container {
    &GLOBAL_CONTAINER
}

/// Initialize services in the container
pub fn init_container() {
    use crate::infrastructure::logging;

    // Register the logger service
    container().register(logging::Logger::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_register_and_resolve() {
        let container = Container::new();
        container.register(42i32);

        assert_eq!(container.resolve::<i32>(), Some(42));
    }

    #[test]
    fn test_container_factory() {
        let container = Container::new();
        container.register_factory(|| "hello".to_string());

        assert_eq!(container.resolve::<String>(), Some("hello".to_string()));
    }
}
