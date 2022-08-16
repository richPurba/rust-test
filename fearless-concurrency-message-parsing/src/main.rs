use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // this val has been owned by send() above
    });

    // recv() is a blocking method
    // try_recv() is a non-blocking method
    let received = rx.recv().unwrap();
    println!("Got: {} ", received);

    ///
    /// with multiple data
    ///
    ///
    ///
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received2 in rx2 {
        println!("Got again: {}", received2)
    }

    ///
    /// no multiple producers
    ///
    ///
    // define a new channel
    let (tx3, rx3) = mpsc::channel();

    let tx_a = tx3.clone(); // clone that newly created channel.

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx_a.send(val).unwrap(); // the cloned channeld sends something
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx3.send(val).unwrap(); // the original channel also sends something
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received3 in rx3 {
        println!("Got the third time: {} ", received3); // do we get the cloned and the original at the same time?
                                                        // the order could be different everytime you run this code, depending on the
                                                        // prioritization of your OS on these threads.
    }
}
