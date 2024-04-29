pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another(){
        panic!("This test will fail");
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{width: 8, height: 6};
        let smaller = Rectangle{width: 5, height: 4};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{width: 8, height: 6};
        let smaller = Rectangle{width: 5, height: 4};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Alice");
        assert!(
            result.contains("Alice"),
            "Greeting did not contain name, value was: {}",result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100(){
        Guess::new(101);
    }



    #[test]
    fn it_works() -> Result<(), String>{
        if 2 + 2 == 4{
            Ok(())
        } else{
            Err(String::from("2 + 2 should equal 4"))
        }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width < other.width && self.height > other.height
    }
}

pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value:i32) -> Guess{
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        }else if value > 100 {
            panic!(" Guess value must be less than or equal to 100, got {}", value);
        }
    Guess{value}
    }
}