use std::{sync::mpsc::channel, thread};

// channels - conept where data is sent from one thread to another
// 2 halves transmitter and reciever - one part calls the code to transmit a message while
// the other part checks and recieves
pub fn channel_impl() {
    let (tx, tr) = channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = tr.recv().unwrap();
    println!("Got: {}", received);
}
