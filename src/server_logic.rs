use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = get_http_request(buf_reader);

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn get_http_request(buf_reader: BufReader<&mut TcpStream>) -> Vec<String> {
    let http_request: Vec<_> = buf_reader
        .lines()
        // todo: error handling
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    http_request
}
