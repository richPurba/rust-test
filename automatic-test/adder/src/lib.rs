/// The #[cfg(test)] annotation on the tests module tells Rust to compile and run
/// the test code only when you run cargo test, not when you run cargo build
#[cfg(test)]
mod tests {

    use super::*; //NOTE Rectangle is outside of this scope


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn is_equal_four(){
        assert_eq!(4, add_two(2));
    }
    /// NOTE ON Expected vs Actual in test cases
    /// However, in Rust, they’re called left and right,
    /// and the order in which we specify the value we expect and
    /// the value that the code under test produces doesn’t matter.
    /// We could write the assertion in this test as assert_eq!(add_two(2), 4),
    /// which would result in a failure message that displays assertion failed: `(left == right)`
    /// and that left was 5 and right was 4.
    ///
    ///
    /// NOTE ON important traits (PartialEq and Debug) to be put if you want to run your test.
    /// which means the values being compared must implement the PartialEq and Debug traits.
    /// All the primitive types and most of the standard library types implement these traits.
    /// For structs and enums that you define, you’ll need to implement PartialEq to assert that
    /// values of those types are equal or not equal.
    /// You’ll need to implement Debug to print the values when the assertion fails.
    /// Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5,
    /// this is usually as straightforward as adding the #[derive(PartialEq, Debug)]

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }

    pub fn add_two(i: i32) -> i32 {
        i + 2
    }

    ///
    /// # Examples
    /// ```
    /// let string = String::from("bla");
    /// let actual = crate::greeting(string);
    /// assert_eq!(actual,String::from("hello");
    /// ```
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }


    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
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


pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess{
            value
        }
    }
}



