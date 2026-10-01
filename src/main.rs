use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*},
};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("server is listening adress now!");

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("client is online now");
        
    }
}

fn handle_connection_stream(mut stream: TcpStream){
    
}