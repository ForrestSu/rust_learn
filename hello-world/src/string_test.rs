#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

#[test]
fn string_test() {
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    println!("Hello World!{:?}", site);
}
