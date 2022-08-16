#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string2, string1.as_str.as_str.as_str.as_str()()()());

        println!("The longest string is {}", result)
    }

    fn longest(x: &str, y:&str ) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
}

}

