#[allow(dead_code)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(_name: &str) -> String {
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {value}
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        // let result = add(2, 2);
        assert_eq!(2+2, 4);
    }
    /* 
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    */
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width:5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    fn _greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
            );
    }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}