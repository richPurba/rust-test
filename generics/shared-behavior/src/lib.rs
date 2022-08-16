#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        let s = 3.to_string();// NOTE: this is because Integer has Display trait
        // This is an example of blanket implementation
    }

}


struct Pair<T> {
    x: T,
    y: T,
}


impl <T> Pair<T>{
    fn new(x:T, y: T) -> Self{
        Self{x,y}
    }

}
/// the same impl but different trait: a type now can have certain attributes which are required to do the calculation
impl <T: Display + PatialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.y > self.x {
            println!("The largest member is y {}", self.y)
        }
    }
}