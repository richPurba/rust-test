fn main() {
    println!("Hello, world!");
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum Message {
        Quit,
        Move{ x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    impl Message {
        fn call(&self) {

        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();


    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // does not compile!

    enum Coin {
        Penny,
        Nickel,
        Dime, 
        Quarter(UsState)
    }

    #[derive(Debug)]
    enum UsState {
        Alabama, Alaska
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 2,
            Coin::Dime => 10,
            Coin::Quarter(state) =>{
                println!("State quarter from {:?}!", state);// you have to provide debug trait for it to run.
                25
            },
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, 
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn move_player(num_spaces: u8) {}


    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
