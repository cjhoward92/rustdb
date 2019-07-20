use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
pub struct Key {
  key: String,
}

impl Key {
  pub fn new(key: &str) -> Key {
    Key {
      key: String::from(key),
    }
  }
}

pub fn hash_key(key: &Key) -> u64 {
  let mut hasher = DefaultHasher::new();
  key.hash(&mut hasher);
  hasher.finish()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_key_new() {
    let new_key = String::from("thisisakey");
    let key = Key::new(&new_key);
    assert_eq!(key.key, new_key);
  }

  #[test]
  fn test_hash_key() {
    let new_key = String::from("thisisakey");
    let key = Key::new(&new_key);
    let hash = hash_key(&key);
    assert!(hash > 0);
  }
}
