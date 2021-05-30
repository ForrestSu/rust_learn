use std::str;

// strtok(s = "hello world", " ")
// return "hello", s = "world"

pub fn strtok<'a>(s: &'a mut &str, pattern: char) -> &'a str {
    match s.find(pattern) {
        Some(i) => {
            let prefix = &s[..i]; //
            let suffix = &s[i + 1..];
            *s = suffix;
            prefix
        }
        None => {
            let prefix = *s;
            *s = "";
            prefix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "hello world";

        let s1 = &mut s;
        let t = strtok(s1, ' ');

        assert_eq!(t, "hello");
        assert_eq!(*s1, "world");

        println!("{:?}", s);
    }
}
