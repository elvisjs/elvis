//! Closure
use std::sync::Arc;

/// store closures
pub type Closure<T> = Arc<Box<dyn FnMut(T) -> u8 + Send + Sync>>;
