use crate::daemon;
use std::{
    io::{prelude::*, BufReader, ErrorKind},
    net::{TcpListener, TcpStream},
};

pub fn start_server(address: String) {
    println!("🤖Agents server attempting to listen at {}", address);

    daemon::initialize_daemon();
    match TcpListener::bind(&address) {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_connection(stream),
                    Err(e) => eprintln!("Failed to handle incoming connection: {}", e),
                }
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::AddrInUse {
                eprintln!("🤖Agents is already running silly :p.");
                return;
            } else {
                eprintln!("Failed to bind to the address: {}", e);
                return;
            }
        }
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("received request {:#?}", http_request);
    let response = "HTTP/1.1 200 OK \r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
