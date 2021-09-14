#![cfg_attr(feature = "unstable", feature(test))]

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    let serial = serde_json::to_string(&point).unwrap();
    println!("serial {}", serial);

    let deserial: Point = serde_json::from_str(&serial).unwrap();
    println!("deserial {:?}", deserial);

    let start = SystemTime::now();
    println!("start= {:?}", start);
}

#[test]
fn bench_new() {
    let point = Point { x: 10, y: 20 };
    let serial = serde_json::to_string(&point).unwrap();

    let start = SystemTime::now();
    println!("serial {:?}", serial);
    for _ in 0..1000000 {
        let deserial: Point = serde_json::from_str(&serial).unwrap();
    }
    let end = SystemTime::now();
    println!("cost {:?}", end.duration_since(start));
}
