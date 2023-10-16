use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::str::FromStr;
use std::time::Duration;
use http::{Method, Version};

use http_client::request::{Command, Entry, Request, RequestLine};
use http_client::{http_header, request};
use url::{Host, Url};

struct Cli {
    version: Option<Version>,
    url: Option<Url>,
    /// todo:去重
    headers: Vec<Entry>,
    data: Option<String>,
    method: Option<Method>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let mut commands = vec![];
    let mut i = 1;
    while i < args.len() {
        let command = match args[i].as_str() {
            "-0" => Command::Version(Version::HTTP_10),
            "-X" => {
                i += 1;
                Command::Method(args[i].to_string())
            },
            "-H" => {
                i += 1;
                Command::Header(args[i].to_string())
            },
            "-d" => {
                i += 1;
                Command::Data(args[i].to_string())
            },
            "-b" => {
                i += 1;
                Command::Cookie(args[i].to_string())
            },
            url => Command::Url(url.to_string()),
        };
        commands.push(command);
        i += 1;
    }
    // println!("{:?}", commands);

    let accept = Entry {
        key: http_header::ACCEPT.to_string(),
        value: "*/*".to_string(),
    };

    let accept_encoding = Entry {
        key: http_header::ACCEPT_ENCODING.to_string(),
        value: "gzip, deflate, br".to_string(),
    };

    let connection = Entry {
        key: http_header::CONNECTION.to_string(),
        value: "keep-alive".to_string(),
    };

    let mut cli = Cli {
        version: None,
        url: None,
        headers: vec![accept, accept_encoding, connection],
        data: None,
        method: None,
    };

    for command in commands {
        match command {
            Command::Version(ver) => {
                cli.version = Some(ver);
            },
            Command::Url(url_str) => {
                let url_parse: Url = Url::parse(url_str.as_str()).expect("url is illegal");
                cli.url = Some(url_parse);
            },
            Command::Header(header_line) => {
                match header_line.find(":") {
                    None => panic!("Header: {header_line}  is illegal"),
                    Some(index) => {
                        let (key, value) = header_line.split_at(index);
                        let header = Entry {
                            key: key.trim().to_string(),
                            value: value.trim().to_string(),
                        };
                        cli.headers.push(header);
                    }
                }

            },
            Command::Data(data_str) => cli.data = Some(data_str),
            Command::Form { .. } => {}
            Command::Method(method_str) => cli.method = Some(Method::from_str(method_str.as_str()).expect("method is illegal")),
            Command::Cookie(cookie) => {
                let header = Entry {
                    key: http_header::COOKIE.to_string(),
                    value: cookie,
                };
                cli.headers.push(header);
            }
            Command::CookieJar { file_name } => {}
            Command::JunkSessionCookies => {}
            Command::OutPutHerder => {}
            Command::OutPutHerderAndData => {}
            Command::DumpHeader { file_name } => {}
        }
    }

    let url = cli.url.expect("url is not found");
    let socket_addrs = url.socket_addrs(|| None).expect("url can not cast to socket_addrs");
    let path = url.path().to_string();
    let host = match url.host() {
        None => panic!("host is missing"),
        Some(host) => {
            let port = url.port();
            match (host, port) {
                (Host::Domain(domain), Some(port)) => {
                    format!("{}:{}", domain.to_string(),port.to_string())
                }
                (Host::Domain(domain), None) => domain.to_string(),
                (Host::Ipv4(addr), Some(port)) => {
                    format!("{}:{}", addr.to_string(),port.to_string())
                },
                (Host::Ipv4(addr), None) => addr.to_string(),
                (Host::Ipv6(_),_) => panic!("Ipv6 is not support"),
            }
        }
    };
    cli.headers.push(Entry {
        key: http_header::HOST.to_string(),
        value: host.clone(),
    });

    let version = match cli.version {
        None => Default::default(),
        Some(version) => version
    };

    let protocol = match version {
        Version::HTTP_10 => request::HTTP_10,
        Version::HTTP_11 => request::HTTP_11,
        _ => panic!("http version is not support"),
    }.to_string();

    let request = Request {
        request_line: RequestLine {
            method: match cli.method {
                None => Default::default(),
                Some(method) => method
            },
            path,
            protocol,
        },
        request_header: cli.headers,
        request_data: cli.data,
    };
    // println!("{:?}", request);

    let http_message = request.to_message();
    // println!("{}", http_message);

    let mut stream = TcpStream::connect(&socket_addrs[..])
        .expect("Couldn't connect to the server...");

    stream.set_write_timeout(Some(Duration::new(10, 0)))
        .expect("set_write_timeout call failed");

    stream.set_read_timeout(Some(Duration::new(10, 0)))
        .expect("set_read_timeout call failed");

    stream.write(http_message.as_bytes())
        .expect("write failed");

    let mut buffer = [0; 1024 * 1024];
    stream.read(&mut buffer)
        .expect("read failed");
    let response = String::from_utf8_lossy(&buffer);
    let response = response.trim_matches('\u{0}');
    println!("response: {:?}", response);

    Ok(())
}

