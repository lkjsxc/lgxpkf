use crate::config::Config;
use crate::storage::Storage;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub storage: Storage,
}
