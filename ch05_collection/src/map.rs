#[test]
fn test_map() {
    use std::collections::HashMap;
    // test hashMap
    let mut map = HashMap::new();
    map.insert(100, "Alice");
    println!("len = {}", map.len());
    println!("capacity = {}", map.capacity());
    for (key, val) in map {
        println!("key:{}, value:{}", key, val);
    }
}
