use super::handler;
use crate::daemon;
use crate::db::DbConfig;
use pickledb::PickleDb;
use std::{
    io::ErrorKind,
    net::TcpStream,
    sync::{Arc, Mutex},
};
use warp::Filter;

pub fn start_server(port: &String, attatch: &bool, mut db: DbConfig) {
    db.config_db.set("port", port).unwrap();
    println!("Server attempting to listen on {}", port);

    if !attatch {
        daemon::initialize_daemon();
    }

    let port: u16 = port.parse().expect("Invalid port number");
    initialize_server(port, db.agents_db);
}

#[tokio::main]
async fn initialize_server(port: u16, db: PickleDb) {
    let db = Arc::new(Mutex::new(db));

    let ui_db = db.clone();
    let ui = warp::path::end()
        .and(warp::any().map(move || ui_db.clone()))
        .and_then(handler::ui::ui);

    let get_agent_db = db.clone();
    let get_agent = warp::path("agent")
        .and(warp::path::param())
        .and(warp::any().map(move || get_agent_db.clone()))
        .and_then(handler::agents::get_agent);

    let list_agents_db = db.clone();
    let list_agents = warp::path("agent")
        .and(warp::any().map(move || list_agents_db.clone()))
        .and_then(handler::agents::list_agents);

    let routes = warp::get().and(ui.or(get_agent).or(list_agents));
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

pub fn status(db: &mut PickleDb) {
    let port = db.get::<String>("port");
    let port = match port {
        Some(port) => port,
        None => String::from("7979"),
    };
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
