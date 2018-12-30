trait Talker {
    fn get_text() -> String;
}

struct Greeter {}
struct Lecturer {}

impl Talker for Greeter {
    fn get_text() -> String {
        String::from("Hello everybody!")
    }
}

impl Talker for Lecturer {
    fn get_text() -> String {
        String::from("Hello students!")
    }
}

fn main() {
    println!("{}", Greeter::get_text());
    println!("{}", Lecturer::get_text());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_greeter() {
        assert_eq!(Greeter::get_text(), "Hello everybody!");
    }

    #[test]
    fn test_lecturer() {
        assert_eq!(Lecturer::get_text(), "Hello students!");
    }
}