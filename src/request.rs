use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use http::{Method};

pub static HTTP_10: &str = "HTTP/1.0";
pub static HTTP_11: &str = "HTTP/1.1";

/// todo
/// --https
/// --tlsv1
/// --tlsv1.0
/// --tlsv1.1
/// --tlsv1.2
/// --sslv2
/// --sslv3

#[derive(Debug)]
pub struct Request {
    pub socket_addrs: Vec<SocketAddr>,
    pub request_line: RequestLine,
    pub request_header: Vec<Entry>,
    pub request_data: Option<String>,
}

impl Request {
    pub fn to_message(&self) -> String {
        let mut res = String::new();
        res.push_str(self.request_line.to_message().as_str());
        for entry in &self.request_header {
            res.push_str(entry.to_message().as_str());
        }
        res.push_str("\r\n");
        if let Some(data) = &self.request_data {
            res.push_str(data);
        }
        res
    }

    pub fn do_http(&self) {
        let http_message = self.to_message();
        println!("request: {:?}", http_message);

        let mut stream = TcpStream::connect(&self.socket_addrs[..])
            .expect("Couldn't connect to the server...");

        stream.set_write_timeout(Some(Duration::new(10, 0)))
            .expect("set_write_timeout call failed");

        stream.set_read_timeout(Some(Duration::new(10, 0)))
            .expect("set_read_timeout call failed");

        stream.write(&http_message.as_bytes())
            .expect("write failed");

        let mut buffer = [0; 1024 * 1024];
        stream.read(&mut buffer)
            .expect("read failed");
        let response = String::from_utf8_lossy(&buffer);
        let response = response.trim_matches('\u{0}');
        println!("response: {:?}", response);
    }
}

#[derive(Debug)]
pub struct RequestLine {
    pub method: Method,
    pub path: String,
    pub protocol: String,
}

impl RequestLine {

    pub fn to_message(&self) -> String {
        format!("{} {} {}\r\n", self.method, self.path, self.protocol)
    }
}

#[derive(Debug)]
pub struct Entry {
    pub key: String,
    pub value: String,
}

impl Entry {
    pub fn to_message(&self) -> String {
        format!("{}: {}\r\n", self.key, self.value)
    }
}
