pub mod agents {
    use crate::agent::agent::Agent;
    use pickledb::PickleDb;
    use std::convert::Infallible;
    use std::sync::{Arc, Mutex};
    use warp::http::StatusCode;
    use warp::reply::{self};

    pub async fn get_agent(
        agent_name: String,
        db: Arc<Mutex<PickleDb>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let db = db.lock().unwrap();

        let agent = db.get::<Agent>(&agent_name);
        match agent {
            Some(agent) => Ok(reply::with_status(reply::json(&agent), StatusCode::OK)),
            None => Ok(reply::with_status(
                reply::json(&"Agent not found"),
                StatusCode::BAD_REQUEST,
            )),
        }
    }
}

pub mod ui {
    use crate::agent::agent::Agent;
    use minijinja::{context, Environment};
    use pickledb::PickleDb;
    use serde::Serialize;
    use std::convert::Infallible;
    use std::sync::{Arc, Mutex};
    use warp::reply;

    #[derive(Serialize)]
    pub struct Page {
        title: String,
        content: String,
        agents: Vec<Agent>,
    }

    pub async fn ui(db: Arc<Mutex<PickleDb>>) -> Result<reply::Html<String>, Infallible> {
        let mut env = Environment::new();
        env.add_template("index.html", include_str!("templates/index.html"))
            .unwrap();

        env.add_template("layout.html", include_str!("templates/layout.html"))
            .unwrap();

        let db = db.lock().unwrap();
        let mut agents: Vec<Agent> = Vec::new();
        for agent_iter in db.get_all() {
            if let Some(curr_agent) = db.get::<Agent>(&agent_iter) {
                agents.push(curr_agent);
            } else {
                println!("Attempted to access invalid agent {}", agent_iter);
            }
        }
        let template = env.get_template("index.html").unwrap();

        let page = Page {
            title: "Some title".into(),
            content: "Some content".into(),
            agents: agents.into(),
        };
        let rendered = template.render(context!(page)).unwrap();
        Ok(reply::html(rendered))
    }
}
