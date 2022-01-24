fn main() {
    let mut counter = 0;
   let result = loop {
       counter += 1;
       if counter == 10 {
           // break counter * 2;
           break
           counter * 2

           //important here that you want to break and give value to the assignment
           //result! so there is no semi-colon after break, such that the return is counter*2
           // This is important because you don't have to put another if statement to break or switch
       }
   };
    println!("the result is {}",result);


    let mut number = 3;

    while number != 0 {
        println!("{}!",number);
        number -=1;
    }

    println!("LIFTOFF!!!");


    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("the value is: {}", element)
    }

    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}
