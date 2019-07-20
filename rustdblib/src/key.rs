use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Result as FmtResult};
use std::hash::{Hash, Hasher};

const MAX_KEY_LENGTH: usize = 255;

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

#[derive(Debug)]
pub enum KeyError {
  NullKey,
  BlankKey,
  ExceedsMaxLength(usize),
}

impl fmt::Display for KeyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> FmtResult {
    match self {
      KeyError::NullKey => write!(f, "{:?} - key cannot be null", self),
      KeyError::BlankKey => write!(f, "{:?} - key cannot be blank", self),
      KeyError::ExceedsMaxLength(max) => write!(f, "{:?} - key cannot exceed max length of {}", self, max),
    }
  }
}

fn validate_key(key: &Key) -> Result<bool, KeyError> {
  let key_slice = &key.key[..];
  let key_len = key_slice.trim().len();
  if key_len == 0 {
    return Err(KeyError::BlankKey)
  } else if key_len > MAX_KEY_LENGTH {
    return Err(KeyError::ExceedsMaxLength(MAX_KEY_LENGTH))
  }

  Ok(true)
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
