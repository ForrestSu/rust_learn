#[cfg(test)]
mod tests {
    #[test]
    fn hello_test() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn test_panic() {
    panic!("I am panic {}", 10);
}
