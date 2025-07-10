use crate::config::Config;
use std::sync::Arc;

/// Global application state shared across routes and handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
}
