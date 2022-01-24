fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const is UPPERCASE with underscore by convention!
    const MAX_POINTS: u32 = 100_000;

    let x = x + 1;
    let x = x * 2;
    println!("the value of x is: {}",x);


    // creating guess without annotation ( `:u32` ) will create compile-error
    // let guess = "42".parse().expect("WHa");
    let guess :u32 = "42".parse().expect("WHa");


    let xx = 2.0; //f64
    let yy: f32 = 3.0; // f32

    let t = true;
    let f: bool = false;

    // char has 4 bytes in size.
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';


    // TUPLES, a fixed collection with different type, and you use parenthesis.
    let tup: (i32, f64, u8) = (500, 6.4, 2);
    let (x,y,z) = tup;
    println!("the value of the tuple is: {}",y);

    let x:(i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; // accessing indices of the tuple
    let six_point_four = x.1;

    // ARRAY , has to be the same type
    let a = [1 , 2, 3, 4,]; // look at the last comma.
    let a :[i64;3] = [12, 1200000000, 54]; // see that the number of element is defined in the
                                        // annotation!
    let short = [3;5]; // initializing an array with 3 for 5 elements
    println!("this is the short version of initializing an array: {}", short[0]);// if you type `short.0` it doesn't give compile error


    let a = [1,2,3,4,5];
    let index = 10;

    // let element = a[index];
    // println!("the value of element is: {}",element);

    another_function(5,"na");

    let it_returns = function_that_returns();
    println!("it_returns returns: {}",it_returns);

    let plus = plus_one(5);
    println!("plus is {}",plus);
}

/// by convention, function is with snake (lowercase with _ )
fn another_function(x: i32, y:&str){ // super awesome, String is pointer.
    println!("Another function with value: {} and {}",x,y);

    let x = 5;
    let y = { // this is calling an expression
        let x = 4;
        x+1  // funny that when you put ; this doesn't return anything and println! below won't work
    };
    println!("the value of y is: {}",y);
}


fn function_that_returns() -> i32 {
    14
}


fn plus_one(x: i32) -> i32 {
    x+1
}

