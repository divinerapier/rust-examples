#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("{}", 1);
    }

    // #[derive(Debug)] // procedu macro
    struct User {
        name: String,
    }

    use std::fmt::Display;

    impl Display for User {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "user's name: {}", self.name)
        }
    }

    #[test]
    fn test2() {
        use std::fmt::Debug;
        let u = User {
            name: String::from("Tom"),
        };
        println!("{}", u);
    }
}
