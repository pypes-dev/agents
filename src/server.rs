use crate::agent::agent;
use crate::daemon;
use minijinja::{context, Environment};
use pickledb::PickleDb;
use serde::Serialize;
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

    let routes = warp::any().map(move || {
        let db = db.lock().unwrap();
        let mut agents: Vec<agent::Agent> = Vec::new();
        for agent_iter in db.liter("agents") {
            let curr_agent = agent_iter.get_item::<agent::Agent>().unwrap();
            agents.push(curr_agent);
        }
        let template = env.get_template("index.html").unwrap();

        let page = Page {
            title: "Some title".into(),
            content: "Some content".into(),
            agents: agents.into(),
        };
        let rendered = template.render(context!(page)).unwrap();
        warp::reply::html(rendered)
    });
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
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
