use generics::longest_with_lifetime;

mod lib;

fn main() {
    let number_list = vec![34,50,23,100];
    //TODO:
    let result = largest(&number_list);
    println!("The largest from number_list is {}", result);

    let a1: f32 = 1.2;
    let a2: f32 = 2.5;
    let integer = Point{x:5,y:10};
    let float = Point{x: a1, y: a2};

    println!("the value of integer's field x: {}", integer.x());
    println!("distance from origin: {}", float.distance_from_origin());

    let p1 = PointAgain{ x: 5, y: 10.5};
    let p2 = PointAgain{x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {} and p3.y {}", p3.x, p3.y);



    let string1 = String::from("abcd");
    let string2 = "xyz";

    // let result = lib::longest(string2, string1.as_str());
    let result = lib::longest_with_lifetime(string2, string1.as_str());
    println!("The longest string is {}", result);


    /// example of lifetime
    let string3 = String::from("xysasbc");
    let result2;
    {
        // let string4 = String::from("000");// string4 owns the object
        let string4 = "22"; // solution where we create the object, not as a reference!
       result2 = longest_with_lifetime(string3.as_str(), string4) ;// we then borrow it here
    }
    println!("The longest string is {}", result2); // but it is being borrowed again

    /// hole in lifetime
    let mut x = Box::new(43);
    let mut z = &x;
    for i in 1..100{
        println!("{}", z);
        x = Box::new(i);
        z = &x;
    }
    println!("Z is {}",z);

    let novel = String::from("call me ishmael. Some years ago....");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = lib::ImportantExcerpt {
        part: first_sentence,
    };

    /// Combining Type parameters, trait bounds and lifetimes together
    ///
    let j = lib::longest_with_an_announcement(string1.as_str(), string2, "WHAAA");
    println!("Value of j is {}", j);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T { // returns a reference of type T
        &self.x
    }
}

impl Point<f32> {
fn distance_from_origin(&self) -> f32{
    (self.x.powi(2) + self.y.powi(2)).sqrt()
}
}

struct PointAgain<X1,Y1> {
    x: X1,
    y: Y1,
}

impl<X1,Y1> PointAgain<X1,Y1>{
    fn mixup<X2,Y2>(self, other: PointAgain<X2,Y2>) -> PointAgain<X1,Y2> {
        PointAgain {
            x: self.x,
            y: other.y,
        }
    }
}