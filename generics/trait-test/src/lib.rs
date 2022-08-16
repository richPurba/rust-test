#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait Summary {
    fn summarize(&self) -> String ;
    fn implemented(&self) -> String {
        format!("Read more .... {}", self.summarize()) // default implementation of a trait.
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub contact: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author,self.location);
    }
}

pub fn notify(item: &impl Summary){
        println!("Breaking news!{}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T){
    println!("test: {}", item.summarize())
}

pub fn notify3(item: &(impl Summary + Display)){

}
pub fn notify4<T: Summary + Display>(item: &T){}

/// Using where clause
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}
/// alternative
pub fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}
/// returns an impl Trait as a return type
pub fn return_summarizable() -> impl Summary {
    NewsArticle{
        headline: String::from("h"),
        location: String::from(""),
        author: String::from("a"),
        contact: String::from("as")
    }
}


/// Example of accepting a reified object in the argument.
pub fn main(){
    let a = NewsArticle{
        headline: String::from("s: &str"),
        location: String::from(""),
        author: String::from("12"),
        contact: String::from("cpont"),
    };
    notify(&a);
}