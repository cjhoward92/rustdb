use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_key_new() {
    let new_key = String::from("thisisakey");
    let key = Key::new(&new_key);
    assert_eq!(key.key, new_key);
  }
}
