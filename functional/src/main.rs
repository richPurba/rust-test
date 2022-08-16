use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::convert::Infallible;

fn main() {
    // TODO: revisit 13.1 March 5th
    let example_closure = |xx| xx;
    let test = example_closure(String::from("s: &str"));
    // let test_again = example_closure(2);// Compile error: the type has been inferred to be String from line above

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // move in Fn ---> still don't understand this
    let x1 = vec![1,2,3];
    let equal_to_x1 = move |z1| z1 == x1;
    let y1 = vec![1,2,3];
    assert!(equal_to_x1(y1));


    let v1: Vec<i32> = vec![1,2,3];
    v1.iter().map(|x|x+1);// will show warning since this is not consumed
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2,3,4])
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if (intensity < 25) {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32, {
    // holdValue: V,
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32,{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
    // fn new(calculation: V) -> Cacher<K,V> {
    //     Cacher {
    //         holdValue: calculation,
    //         calculation: HashMap::new(),
    //         value: None
    //     }
    // }
    //
    // fn value(&mut self, arg: K) ->K {
    //     // match self.value {
    //     //     Some(v) => self.calculation.get(arg),
    //     //     None => {
    //     //         let v = (self.calculation)(arg,self.holdValue); // how to use lambda for Hashmap insert?
    //     //         self.value = Some(v);
    //     //         v
    //     //     }
    //     // }
    //     //TODO revisit March 5th and check how to use HashMap
    //     arg
    // }
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);// failed becaue c.value(2) does not change the value 1.

}


#[test]
fn test_iterator(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum(){
    let v1  = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}


// Series of items
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filters_by_size(){
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size =shoe_in_size(shoes,10);
        assert_eq!(in_my_size, vec![Shoe {
            size: 10, style: String::from("sneaker")
        },
        Shoe { size: 10, style: String::from("boot")}])
    }
}
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}
impl Iterator for Counter {
    type Item = u32; // associated type

    fn next(&mut self) -> Option<Self::Item> {
        if(self.count < 5 ){
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods(){
    let sum: u32 =  Counter::new()
        .zip(Counter::new().skip(1))
        .peekable()
        .map(|(a,b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}