fn main() {
    let number = 3;

    if number < 5
     // this is an arm
    {println!("condition was true"); }
    else // this is an arm
    {println!("condition was false"); }

    // using match is another way of replacing too many if-else statements

    let condition = true;
    let number = if condition { 5 } else { 6}; // very consistent.
}
