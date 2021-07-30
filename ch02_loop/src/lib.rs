#[test]
fn test_loop() {
    // 测试循环
    let list = vec![10, 20, 30, 40];
    for one in list {
        println!("list = {:?}", one);
    }

    for number in (1..4).rev() {
        println!("rev = {}", number);
    }

    let mut index = 0;
    while index < 3 {
        println!("index =  {}", index);
        index += 1;
    }

    println!("fib(5) = {}", fib(5));
}

// 1,1,2,3,5
fn fib(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
    return fib(x - 1) + fib(x - 2);
}
