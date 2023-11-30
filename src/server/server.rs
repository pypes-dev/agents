use super::handler;
use crate::daemon;
use crate::{agent::agent, db::DbConfig};
use minijinja::{context, Environment};
use pickledb::PickleDb;
use serde::Serialize;
use std::{
    io::ErrorKind,
    net::TcpStream,
    sync::{Arc, Mutex},
};
use warp::reply::{self};
use warp::Filter;
pub fn start_server(port: &String, attatch: &bool, mut db: DbConfig) {
    db.config_db.set("port", port).unwrap();

    if !attatch {
        daemon::initialize_daemon();
    }

    let db = Arc::new(Mutex::new(db.agents_db));
    let port: u16 = port.parse().expect("Invalid port number");
    initialize_server(port, db);
}

#[derive(Serialize)]
pub struct Page {
    title: String,
    content: String,
    agents: Vec<agent::Agent>,
}
#[tokio::main]
async fn initialize_server(port: u16, db: Arc<Mutex<PickleDb>>) {
    let mut env = Environment::new();
    env.add_template("index.html", include_str!("templates/index.html"))
        .unwrap();

    env.add_template("layout.html", include_str!("templates/layout.html"))
        .unwrap();
    let db_clone = db.clone();
    let ui = warp::path::end().map(move || {
        let db = db_clone.lock().unwrap();
        let mut agents: Vec<agent::Agent> = Vec::new();
        for agent_iter in db.get_all() {
            let curr_agent = db.get::<agent::Agent>(&agent_iter).unwrap();
            agents.push(curr_agent);
        }
        let template = env.get_template("index.html").unwrap();

        let page = Page {
            title: "Some title".into(),
            content: "Some content".into(),
            agents: agents.into(),
        };
        let rendered = template.render(context!(page)).unwrap();
        reply::html(rendered)
    });
    let get_agent = warp::path("agent")
        .and(warp::path::param())
        .and(warp::any().map(move || db.clone()))
        .and_then(handler::agents::get_agent);
    let routes = warp::get().and(ui.or(get_agent));
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
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
