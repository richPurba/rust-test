use std::collections::HashMap;

pub fn testing() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];

    let mut scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name = field_name +"as"; // field_name is now owned by map!

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(val) => println!("value is {} ", val),
        None => println!("No value"),
    }

    for(k,v ) in &scores {
        println!("{}: {}", k,v);
    }


    scores.insert(String::from("Blue"), 25);
    println!("Blue now is {:?}", scores);
    scores.entry(String::from("Yellow")).or_insert(59);


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // for word in text.split()


    ///
    /// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    /// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    /// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically
}