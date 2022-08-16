
// pub fn longest(x: &str, y:&str ) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

use std::fmt::Display;

pub fn longest_with_lifetime<'a>(x: &'a str, y:&'a str ) -> &'a str {
    let s: &'static str = "I have a static lifetime";
    if x.len() > y.len() {
        println!("{}",s);
        return x;
    } else {
        println!("{}",s);
        return y;
    }
    println!("{}",s);
    return x;
}

fn test<'a>(first: &'a i32, second: &'a i32) -> &'a str{
   // indicates the the reference first and second must both live as long as the generic lifetime
    ""
}
/// 3 Elision Rules
///
/// The first rule is that each parameter that is a reference gets its own lifetime parameter.
/// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
/// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
///
/// The second rule is if there is exactly one input lifetime parameter,
/// that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
///
/// The third rule is if there are multiple input lifetime parameters,
/// but one of them is &self or &mut self because this is a method,
/// the lifetime of self is assigned to all output lifetime parameters.
/// This third rule makes methods much nicer to read and write because fewer symbols are necessary.

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}
impl<'b> ImportantExcerpt<'b> {
    fn level(&self)-> i32 {
        3
    }
}
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display, {
    println!("Announcement! {}", ann);
    if x.len() > y.len(){
        x
    } else {
        y
    }
}