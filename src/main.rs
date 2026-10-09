use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let mut counter = 0;
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("server is listening now!");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        counter += 1;
        println!("client is online now number {counter}");

        handle_connection_stream(stream);
    }
}

fn handle_connection_stream(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);

    let request_lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("request was:\n{request_lines:#?}");

    if !request_lines.is_empty() {
        let (resp, file_path) = match request_lines[0].as_str() {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };
        let contents = fs::read_to_string(file_path).expect("Не удалось прочитать html документ");

        let response = format!(
            "{}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            resp,
            contents.len(),
            contents,
        );

        stream.write_all(response.as_bytes()).unwrap();
    };
}
