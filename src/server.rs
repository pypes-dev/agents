use crate::agent::agent;
use crate::daemon;
use pickledb::PickleDb;
use serde_json::to_string;
use std::{
    io::{prelude::*, BufReader, ErrorKind},
    net::{TcpListener, TcpStream},
};

pub fn start_server(port: &String, attatch: &bool, db: &mut PickleDb) {
    db.set("port", port).unwrap();
    let address = format!("{}:{}", "localhost", port);

    if !attatch {
        daemon::initialize_daemon();
    }

    match TcpListener::bind(&address) {
        Ok(listener) => {
            println!("🤖Agents server attempting to listen at {}", address);
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_connection(stream, db),
                    Err(e) => eprintln!("Failed to handle incoming connection: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Failed to bind to the address: {}", e);

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

pub fn status(db: &mut PickleDb) {
    let port = db.get::<String>("port").unwrap();
    let address = format!("{}:{}", "localhost", port);
    match TcpStream::connect(address.clone()) {
        Ok(_) => {
            println!("Server is running at {}", address);
        }
        Err(e) => {
            if e.kind() == ErrorKind::ConnectionRefused {
                println!("Server is not running.");
            } else {
                eprintln!("Failed to check server status: {}", e);
            }
        }
    }
}

pub fn handle_connection(mut stream: TcpStream, db: &mut PickleDb) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("received request {:#?}", http_request);
    let mut agents: Vec<agent::Agent> = Vec::new();
    for agent_iter in db.liter("agents") {
        let curr_agent = agent_iter.get_item::<agent::Agent>().unwrap();
        agents.push(curr_agent);
    }

    let agents_json = to_string(&agents).unwrap();
    let response = format!("HTTP/1.1 200 OK \r\n\r\n {}", agents_json);

    stream.write_all(response.as_bytes()).unwrap();
}
