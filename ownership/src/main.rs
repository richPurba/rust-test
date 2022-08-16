

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);

    let b = "immutable"; // see intellij suggests that it is type &str, from heap

    let s1 = String::from("hello");
    let s2 = s1;
    // s1 is now out of scope since it is moved to s2. No shallow copy here from the heap.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    // look at these two examples: string from Heap and integer from stack
    let b = String::from("this is a string");
    passing_string(b);// pass by value, like java
    // println!("now can we see b {}",b); // runtime error: value b borrowed after moved.
                                        // b has been used in passing_string() and after {}, it goes
                                        // out of scope and thus this line is runtime error
    let c = 32; // i32 has Copy (a trait!), whereas String is not.
    passing_integer(c);
    println!("can we see c {}",c); // c is still being called from the stack.


    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn passing_string(some_string: String){
    println!("some_string is {}",some_string); // some_string is initialized
}// some_string out of scope and freed from memory

fn passing_integer(int: i32){
    println!("integer is {}",int); // here int is not created but the stack is passed
}

fn gives_ownership() -> String{
    let some_string = String::from("hello");
    some_string // returned
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}
