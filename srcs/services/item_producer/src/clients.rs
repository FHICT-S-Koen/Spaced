use std::{sync::{OnceLock, RwLock}, collections::HashMap};

static USERS: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();
pub fn get_users() -> &'static RwLock<HashMap<String, String>> {
  USERS.get_or_init(|| RwLock::new(HashMap::new()))
}
