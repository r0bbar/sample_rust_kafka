extern crate kafka;

use kafka::producer::{Producer, Record};

fn main() {
    println!("Hello, Kafka Rust!"); // I cannot even hit break point in first line of program (Unless I remove all traces of Kafka reference, then all good.)
    let hosts = vec!["localhost:9092".to_owned(), "localhost:9093".to_owned(), "localhost:9094".to_owned()];

    let mut _producer = Producer::from_hosts(hosts).create().unwrap();

    for i in 0..10 {
        let buf = format!("{i}");
        match _producer.send(&Record::from_value("some-topic", buf.as_bytes())) {
            Ok(()) => {
                println!("Great! message {} sent!", i);
            }
            Err(error) => {
                eprintln!("Error sending message: {}", error);
            }
        }
    }
}
