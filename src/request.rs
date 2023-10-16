use std::fmt::Debug;
use http::{Method, Version};

pub static HTTP_10: &str = "HTTP/1.0";
pub static HTTP_11: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Request {
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
}

#[derive(Debug)]
pub struct RequestLine {
    pub method: Method,
    pub path: String,
    pub protocol: String,
}

impl RequestLine {

    pub fn to_message(&self) -> String {
        let mut res = String::new();
        res.push_str(self.method.as_str());
        res.push_str(" ");
        res.push_str(self.path.as_str());
        res.push_str(" ");
        res.push_str(self.protocol.as_str());
        res.push_str("\r\n");
        res
    }
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
        param: Vec<Entry>,
        /// name=@file, binary
        upload: Vec<Entry>,
        /// name=<file, text
        read: Vec<Entry>,
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

#[derive(Debug)]
pub struct Entry {
    pub key: String,
    pub value: String,
}

impl Entry {
    pub fn to_message(&self) -> String {
        let mut res = String::new();
        res.push_str(self.key.as_str());
        res.push_str(": ");
        res.push_str(self.value.as_str());
        res.push_str("\r\n");
        res
    }
}

// --tlsv1
// --tlsv1.0
// --tlsv1.1
// --tlsv1.2
// --sslv2
// --sslv3
// --http1.0
// --http1.1