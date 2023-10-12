use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() {

}

#[test]
fn server() {
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

#[test]
fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8888")
        .expect("Couldn't connect to the server...");

    stream.set_write_timeout(Some(Duration::new(10, 0)))
        .expect("set_write_timeout call failed");

    stream.set_read_timeout(Some(Duration::new(10, 0)))
        .expect("set_read_timeout call failed");

    stream.write("hello rust".as_bytes())
        .expect("write failed");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer)
        .expect("read failed");
    let response = String::from_utf8_lossy(&buffer);
    let response = response.trim_matches('\u{0}');
    println!("response: {:?}", response)
}