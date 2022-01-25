//
// NOTE: TO create: cargo new --lib restaurant
//
//

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}
mod back_of_house {
    fn fix_incorrect_order(){
        // referring the function out of his scope
        super::eat_at_restaurant();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
use crate::front_of_house::hosting::add_to_waitlist;
use std::collections::HashMap;

fn eat_at_restaurant(){
    add_to_waitlist;
    add_to_waitlist;
    
    let mut map = HashMap::new();
    map.insert(1,2);

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");// note: private field 
}
//   Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other. r.         

use crate::front_of_house::hosting;
//TODO: revisit why i cannot import test_multiple_dir here!