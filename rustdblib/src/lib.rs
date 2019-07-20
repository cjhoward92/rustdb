mod key;

pub fn init() {
    println!("Hello from init!");

    let s = String::from("this is a test key bitch");
    let new_key = key::Key::new(&s);
    let hash = key::hash_key(&new_key);
    println!("This is the hash: {}", hash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_runs() {
        init();
        assert!(true);
    }
}
