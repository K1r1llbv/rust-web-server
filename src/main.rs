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

    let contents = fs::read_to_string("hello.html")
        .expect("Не удалось прочитать hello.html");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();

}