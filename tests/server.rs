use std::io::{Read, Write};
use std::net::TcpListener;

#[test]
fn run() {
    let listener = TcpListener::bind("127.0.0.1:8888")
        .expect("TcpListener bind failed");
    let streams = listener.incoming();

    for stream in streams {
        let mut request = String::new();
        let mut stream = stream
            .expect("Connection client failed");
        stream
            .read_to_string(&mut request)
            .expect("read client failed");

        println!("request: {:?}", request);

        stream.write("hello c".as_bytes())
            .expect("write failed");
    }
}