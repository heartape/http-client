use std::collections::HashMap;
use std::str::FromStr;
use http::{Method, Version};
use url::{Host, Url};
use crate::{http_header, request};

pub struct Cli {
    version: Option<Version>,
    url: Option<Url>,
    headers: Vec<request::Entry>,
    data: Option<String>,
    method: Option<Method>,
}

/// Command
#[derive(Debug)]
pub enum Command {
    Url(String),
    /// -v
    Version(Version),
    /// -H
    Header(String),
    /// -d
    Data(String),
    /// -F
    Form {
        /// name=alan
        param: Vec<request::Entry>,
        /// name=@file, binary
        upload: Vec<request::Entry>,
        /// name=<file, text
        read: Vec<request::Entry>,
    },
    /// -X
    Method(String),
    /// -b
    Cookie(String),
    /// -c
    CookieJar {
        file_name: String,
    },
    /// -j
    JunkSessionCookies,
    /// -l
    OutPutHerder,
    /// -i
    OutPutHerderAndData,
    /// -D
    DumpHeader {
        file_name: String,
    },
}

pub fn parse(args: Vec<String>) -> Cli {
    let commands = parse_to_commands(args);
    commands_to_cli(commands)
}

fn parse_to_commands(args: Vec<String>) -> Vec<Command> {
    let mut commands = vec![];
    let mut i = 0;
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
    commands
}

fn commands_to_cli(commands: Vec<Command>) -> Cli {
    let mut cli = Cli {
        version: None,
        url: None,
        headers: vec![],
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
                        let header = request::Entry {
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
                let header = request::Entry {
                    key: http_header::COOKIE.to_string(),
                    value: cookie,
                };
                cli.headers.push(header);
            }
            Command::CookieJar { file_name: _ } => {}
            Command::JunkSessionCookies => {}
            Command::OutPutHerder => {}
            Command::OutPutHerderAndData => {}
            Command::DumpHeader { file_name: _ } => {}
        }
    }

    let mut header_map = HashMap::new();

    if !header_map.contains_key(http_header::ACCEPT) {
        let accept = request::Entry {
            key: http_header::ACCEPT.to_string(),
            value: "*/*".to_string(),
        };
        header_map.insert(http_header::ACCEPT, accept);
    }

    if !header_map.contains_key(http_header::ACCEPT_ENCODING) {
        let accept_encoding = request::Entry {
            key: http_header::ACCEPT_ENCODING.to_string(),
            value: "gzip, deflate, br".to_string(),
        };
        header_map.insert(http_header::ACCEPT_ENCODING, accept_encoding);
    }

    if !header_map.contains_key(http_header::CONNECTION) {
        let connection = request::Entry {
            key: http_header::CONNECTION.to_string(),
            value: "Keep-Alive".to_string(),
        };
        header_map.insert(http_header::CONNECTION, connection);
    }

    if !header_map.contains_key(http_header::USER_AGENT) {
        let user_agent = request::Entry {
            key: http_header::USER_AGENT.to_string(),
            value: "http-client".to_string(),
        };
        header_map.insert(http_header::USER_AGENT, user_agent);
    }

    if !header_map.contains_key(http_header::CONTENT_TYPE) {
        if let Some(method) = cli.method.clone() {
            match method {
                Method::POST=> { header_map.insert(http_header::CONTENT_TYPE, default_content_type()); }
                Method::PUT=> { header_map.insert(http_header::CONTENT_TYPE, default_content_type()); }
                _ => {}
            }
        }
    }

    if let Some(data) = &cli.data {
        let content_length = request::Entry {
            key: http_header::CONTENT_LENGTH.to_string(),
            value: data.len().to_string(),
        };
        header_map.insert(http_header::CONTENT_LENGTH, content_length);
    }

    let headers = header_map.into_iter().map(|x| x.1).collect();
    cli.headers = headers;
    cli
}

fn default_content_type() -> request::Entry {
    request::Entry {
        key: http_header::CONTENT_TYPE.to_string(),
        value: "application/json; charset=utf-8".to_string(),
    }
}

pub fn cli_to_request(mut cli: Cli) -> request::Request {
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
    cli.headers.push(request::Entry {
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

    let method = match cli.method {
        None => Default::default(),
        Some(method) => method
    };

    request::Request {
        socket_addrs,
        request_line: request::RequestLine {
            method,
            path,
            protocol,
        },
        request_header: cli.headers,
        request_data: cli.data,
    }
}

