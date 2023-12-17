use super::handler;
use crate::daemon;
use crate::db::DbConfig;
use axum::{routing::get, Router};
use pickledb::PickleDb;
use std::{
    io::ErrorKind,
    net::TcpStream,
    sync::{Arc, RwLock},
};

pub fn initialize_server(port: &String, attatch: &bool, mut db: DbConfig) {
    db.config_db.set("port", port).unwrap();

    if !attatch {
        daemon::initialize_daemon();
    }

    let port: u16 = port
        .parse()
        .expect(&format!("Invalid port number {}", port));

    serve(port, db.agents_db);
}

#[tokio::main]
async fn serve(port: u16, db: PickleDb) {
    let db = Arc::new(RwLock::new(db));

    let app = app().with_state(db);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await;

    match listener {
        Err(e) => println!("🦿{}", e),
        Ok(listener) => {
            println!("🤖 Server listening on port {}", port);
            axum::serve(listener, app).await.unwrap()
        }
    }
}

pub fn app() -> Router<Arc<RwLock<PickleDb>>> {
    Router::new().route(
        "/agents",
        get(handler::agents::agents_index).post(handler::agents::agents_create),
    )
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
            println!("🤖 Server is running at {}", address);
        }
        Err(e) => {
            if e.kind() == ErrorKind::ConnectionRefused {
                println!("🤖 Server is not running.");
            } else {
                eprintln!("🦿 Failed to check server status: {}", e);
            }
        }
    }
}
