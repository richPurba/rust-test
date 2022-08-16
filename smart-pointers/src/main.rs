use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
use crate::List::{Cons, Nil};
 
    let b = Box::new(5);
    println!("b = {}", b);
    
    // let list = Cons(1,Cons(2, Cons(3, Nil)));

    let x = 5;
    let y = &x;// pointer to x

    assert_eq!(5,x);
    assert_eq!(5, *y);//derefence

    let yy = MyBox::new(x);// allocate to heap
    assert_eq!(5, *yy); // derefeence yy

    let m = MyBox::new(String::from("Rust"));
 
    hello(&m);
 
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");


    let cc = CustomSmartPointer {
        data: String::from("Some data"),
    };
    println!("CutomSmartPointer smart data created.");
   // cc.drop(); NOTE: would create double free error if we were to use destructor
    drop(cc);
    println!("CutomSmartPointer smart data dropped before the end of the scope main.");

    
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {} ", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {} ", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c {} ", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope {} ", Rc::strong_count(&a));
}

enum List {
    Cons(i32, Rc<List>),
    //Cons(i32, Box<List>),
    Nil,
}
// struct Cons {
//     index: i32,
//     cons: [Cons; 100],
// }

struct Nil {

}

fn hello(name: &str) {
    println!("Hello! {}", name);
}


struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T> {
 
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // when your type goes out of scope. You will see that the line below will be printed after "CustomSmartPointers created."
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


