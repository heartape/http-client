use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use url::Url;

#[test]
fn run() {
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

#[test]
fn host() {
    let url = Url::parse("https://127.0.0.1:8088/index.html").unwrap();
    let domain = url.host_str().unwrap();
    let host = url.host().unwrap().to_string();
    assert!(url.host().is_some());

    let url = Url::parse("ftp://rms@example.com").unwrap();
    assert!(url.host().is_some());

    let url = Url::parse("unix:/run/foo.socket").unwrap();
    assert!(url.host().is_none());

    let url = Url::parse("data:text/plain,Stuff").unwrap();
    assert!(url.host().is_none());
}