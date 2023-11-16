use std::{sync::{OnceLock, RwLock}, collections::HashMap};

// pub static USERS: RwLock<Vec<usize>> = RwLock::new(vec![]);
static USERS: OnceLock<RwLock<HashMap<String, usize>>> = OnceLock::new();
pub fn get_users() -> &'static RwLock<HashMap<String, usize>> {
  USERS.get_or_init(|| RwLock::new(HashMap::new()))
}
