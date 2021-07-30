#[derive(Debug)]
pub struct Student {
    pub name: String,
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: String::from(name),
        }
    }
}

#[test]
fn test_object() {
    let stu = Student::new("sunquan");
    println!("{:?}", stu);
}
