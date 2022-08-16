///
/// this ampersand (&) is a reference. You are allowed to refer to some value without
/// taking ownership of it

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of '{}' is {}",s1,len);// so no need to be owned by calculate_length



    ///
    /// Borrowing mutable reference can only happen once!
    let mut str = String::from("hello");
    modify_str(&mut str);
    println!("now str is {}",str);
    //TODO: IMPORTANT: only one mutable reference to a particular type
    let r1 = &mut str;
    // let r2 = &mut str; // error : cannot borrow `str` as mutable more than once at a time
    // println!("let see if the second assignment of str works: r1 is {}, r2 is {}",r1,r2);

    //TODO solution? scope
    {
        let r2 = &mut str;
        println!("now what is r2? {}", r2);
    }

    ///
    /// Borrowing (immutable) reference
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // also no problem
    println!("{} and {} are okay",r1,r2);

    let r3 = &mut s;
}

fn calculate_length(s: &String) -> usize{
    s.len()
}// doesn't drop s at this line

fn modify_str(s: &mut String) {// Borrower
    // s.push_str(" world!");
    //TODO compile error: references are immutable by default! Or, you can't change something you borrow

    s.push_str(", world!");
}

fn some_test(){
    let s1 = String::from("bla");
    let s2  = &s1;
    let s3 = &s1;
}






fn some(){



}

