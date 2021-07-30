#[derive(Debug)]
enum RoleType {
    User,
    Admin,
}

#[test]
fn test_enum() {
    println!("{:?}", RoleType::User);
}
