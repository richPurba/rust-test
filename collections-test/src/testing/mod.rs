pub fn bar() {

}

pub fn string(){

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial_content".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("test {} ", s);


    let s1= String::from("Hell");
    let s2 = String::from("worl");
    let s3 = s1  + &s2; // s1.add(other: &str)
    // the borrowed value of s1 is #add(&String) !!! This is deref coercion. 

    println!("the string: {}", s3);
    // let s4 = s1; //NOTe this does not compile, since the  s1 has been borrowed already
    let s4 = s3;// this works



    // Indexing
    let s1 = String::from("hellp");

    // graphene cluster

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
 
    
}


