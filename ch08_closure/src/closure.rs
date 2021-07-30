#[test]
fn test_closure() {
    let inc = |num| {
        println!("call...");
        num + 1
    };
    println!("lambda = {}", inc(10));
}
