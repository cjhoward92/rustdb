mod key;

pub fn init() {
    println!("Hello from init!");
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
