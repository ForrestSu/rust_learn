
fn main() {
    println!("Hello, world!");

    let a = 123;
    let a = 456;
    test_string();

    add(1, 2);
}

// 测试字符串
fn test_string() {
    let mut s = "123";
    println!("{}, len = {}", s, s.len())
    // s = s.len();
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let x = add(1, 2);
///
/// ```

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
