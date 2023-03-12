use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct RequestLine {
    method: Option<String>,
    path: Option<String>,
    protocol: Option<String>,
}

impl RequestLine {
    fn method(&self) -> String {
        if let Some(method) = &self.method {
            method.to_string()
        } else {
            String::from("")
        }
    }
    fn path(&self) -> String {
        if let Some(path) = &self.path {
            path.to_string()
        } else {
            String::from("")
        }
    }
    fn get_order_number(&self) -> String {
        let path = self.path();
        let path_tokens: Vec<String> = path.split("/").map(|s| s.parse().unwrap()).collect();
        path_tokens[path_tokens.len() - 1].clone()
    }
}

impl FromStr for RequestLine {
    type Err = ParseError;

    fn from_str(msg: &str) -> Result<Self, Self::Err> {
        let mut msg_tokens = msg.split_ascii_whitespace();
        let method = match msg_tokens.next() {
            Some(token) => Some(String::from(token)),
            None => None,
        };

        let path = match msg_tokens.next() {
            Some(token) => Some(String::from(token)),
            None => None,
        };

        let protocol = match msg_tokens.next() {
            Some(token) => Some(String::from(token)),
            None => None,
        };

        Ok(Self {
            method,
            path,
            protocol,
        })
    }
}
fn main() {
    static PORT: u16 = 3000;

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT);

    let connection_listener = TcpListener::bind(socket).unwrap();

    println!("Running on port:{}", PORT);

    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 200];

        stream.read(&mut buffer).unwrap();
        let req_line = "";

        let string_request_line =
            if let Some(line) = str::from_utf8(&buffer).unwrap().lines().next() {
                line
            } else {
                println!("Invalid request line received");
                req_line
            };

        let req_line = RequestLine::from_str(string_request_line).unwrap();

        //construct the HTTP res string
        let html_response_string: String;
        let order_status;

        println!("Len is {}", req_line.get_order_number().len());

        if req_line.method() != "GET"
            || !req_line.path().starts_with("/order/status/")
            || req_line.get_order_number().len() == 0
        {
            if req_line.get_order_number().len() == 0 {
                order_status = format!("Please provide  valid order number");
            } else {
                order_status = format!("Sorry this page is not found");
            }
            html_response_string = format!(
                "HTTP/1.1 404 Not Found\nContent-Type: text/html \nContent-Length:{}\n\n{}",
                order_status.len(),
                order_status
            )
        } else {
            order_status = format!(
                "Order status for order number {} is: Shipped\n",
                req_line.get_order_number()
            );
            html_response_string = format!(
                "HTTP/1.1 200 OK\nContent-Type: text/html \nContent-Length:{}\n\n{}",
                order_status.len(),
                order_status
            )
        }
        stream.write(html_response_string.as_bytes()).unwrap();
    }
}
