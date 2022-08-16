fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();   


       let v2 = vec![1,2,3];
       
       let v3 = vec![1,2,3,4,5];
       let third: &i32 = &v3[2];
       println!("the third element is {}", third);

       match v2.get(3) {
           Some(third) => println!("The third element is #### {} ", third),
           None => println!("there is no third element!!"),
       }


       let mut vm = vec![1,2,3,4,5];
       let first = &vm[0];
       vm.push(5);// push means allocating new memory to accomodate new data
    //    println!("The first element is {}", first); // this won't compile since the vm is reallocating new memory and the `first` element 
                                                    // which is the first elementin the array is being called here. Rust doesn't allow this 
                                                    // data race issue.

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i );
    }
    let mut vv = vec![100, 32, 57];
    for i in &mut vv {
        *i += 50;  // * is the dereference operator
    }

    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("Bla")),
        SpreadSheetCell::Float(10.12),
    ];


    let mut v_mut = Vec::new();
    v_mut.push(5);


    let v_immute = vec![1,2,3,4,5];
    let third: &i32 = &v[2];

    let does_not_exists = v_immute.get(100);

    let v = vec![100,32,57];
    
    for i in &v {
        println!("{}", i)
    }   

    string();

    testing();

    //// Note on dereference operator
    /// 
    /// type y below is not exactly an integer, but a pointer type ?????
    /// but type x is indeed integer!
    let x = 5;
    let y = &x;   

}
mod testing;
use testing::{bar,string};

mod hash;
use hash::testing;





