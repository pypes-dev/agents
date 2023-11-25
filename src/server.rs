use crate::agent::agent;
use crate::daemon;
use pickledb::PickleDb;
use serde_json::to_string;
use std::{
    io::ErrorKind,
    net::TcpStream,
    sync::{Arc, Mutex},
};
use warp::Filter;

pub fn start_server(port: &String, attatch: &bool, mut db: PickleDb) {
    db.set("port", port).unwrap();

    if !attatch {
        daemon::initialize_daemon();
    }

    let db = Arc::new(Mutex::new(db));
    initialize_server(db);
}

#[tokio::main]
async fn initialize_server(db: Arc<Mutex<PickleDb>>) {
    let routes = warp::any().map(move || {
        let db = db.lock().unwrap();
        let mut agents: Vec<agent::Agent> = Vec::new();
        for agent_iter in db.liter("agents") {
            let curr_agent = agent_iter.get_item::<agent::Agent>().unwrap();
            agents.push(curr_agent);
        }
        let agents_json = to_string(&agents).unwrap();
        return agents_json;
    });
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
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
