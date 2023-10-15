use std::io::{Read, Write};
use std::net::TcpListener;

#[test]
fn run() {
    let listener = TcpListener::bind("127.0.0.1:8888")
        .expect("TcpListener bind failed");
    let streams = listener.incoming();

    for stream in streams {
        let mut buffer = [0; 1024];
        let mut stream = stream
            .expect("Connection client failed");
        stream
            .read(&mut buffer)
            .expect("read client failed");

        let request = String::from_utf8_lossy(&buffer);
        let request = request.trim_matches('\u{0}');
        println!("request: {:?}", request);

        stream.write("hello c".as_bytes())
            .expect("write failed");
    }
}