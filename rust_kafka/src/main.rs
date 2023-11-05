/* 
Kafka for Rust: https://github.com/kafka-rust/kafka-rust
https://blog.logrocket.com/building-rust-microservices-apache-kafka/
https://stackoverflow.com/questions/77420767/vscode-issue-while-debug-rust

OpenSSL on Windows is a bitch:
    cargo build
        Updating crates.io index
    Compiling openssl-sys v0.9.95
    error: failed to run custom build command for `openssl-sys v0.9.95`

    Caused by:
    process didn't exit successfully: `D:\dev\samples\rust\kafka_demo\target\debug\build\openssl-sys-c2690b7cfe32b0fd\build-script-main` (exit code: 101)
    ... OPENSSL_INCLUDE_DIR unset
    cargo:rerun-if-env-changed=X86_64_PC_WINDOWS_MSVC_OPENSSL_DIR
    X86_64_PC_WINDOWS_MSVC_OPENSSL_DIR unset

To fix:
    https://stackoverflow.com/questions/55912871/how-to-work-with-openssl-for-rust-within-a-windows-development-environment
    
    Steps are, note they take a long time:
    clone vcpkg https://github.com/Microsoft/vcpkg
    open directory where you've cloned vcpkg
    run ./bootstrap-vcpkg.bat
    run ./vcpkg.exe install openssl-windows:x64-windows
    run ./vcpkg.exe install openssl:x64-windows-static
    run ./vcpkg.exe integrate install
    run set VCPKGRS_DYNAMIC=1 (or simply set it as your environment variable)
*/ 
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
