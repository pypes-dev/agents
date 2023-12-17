pub mod agents {
    use crate::agent::agent::Agent;
    use axum::{
        extract::{Query, State},
        response::IntoResponse,
        Json,
    };
    use pickledb::PickleDb;
    use serde::{Deserialize, Serialize};
    use std::convert::Infallible;
    use std::sync::{Arc, Mutex, RwLock};
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

    pub async fn list_agents(db: Arc<Mutex<PickleDb>>) -> Result<impl warp::Reply, Infallible> {
        let db = db.lock().unwrap();

        let mut agents: Vec<Agent> = Vec::new();
        for agent_iter in db.get_all() {
            if let Some(curr_agent) = db.get::<Agent>(&agent_iter) {
                agents.push(curr_agent);
            } else {
                println!("Attempted to access invalid agent {}", agent_iter);
            }
        }
        Ok(reply::with_status(reply::json(&agents), StatusCode::OK))
    }

    // The query parameters for todos index
    #[derive(Debug, Deserialize, Default)]
    pub struct Pagination {
        pub offset: Option<usize>,
        pub limit: Option<usize>,
    }
    type Db = Arc<RwLock<PickleDb>>;
    pub async fn agents_index(
        _pagination: Option<Query<Pagination>>,
        State(db): State<Db>,
    ) -> impl IntoResponse {
        let db = db.read().unwrap();
        let mut agents: Vec<Agent> = Vec::new();
        for agent_iter in db.get_all() {
            if let Some(curr_agent) = db.get::<Agent>(&agent_iter) {
                agents.push(curr_agent);
            } else {
                println!("Attempted to access invalid agent {}", agent_iter);
            }
        }
        Json(agents)
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateAgent {
        name: String,
        inputs: Vec<serde_json::Value>,
        actions: Vec<String>,
    }

    #[derive(Debug, Serialize)]
    struct CreateAgentResponse {
        records_created: u8,
    }
    pub async fn agents_create(
        State(db): State<Db>,
        Json(input): Json<CreateAgent>,
    ) -> impl IntoResponse {
        let agent = Agent {
            name: input.name,
            inputs: input.inputs,
            actions: input.actions,
        };
        db.write().unwrap().set(&agent.name, &agent).unwrap();
        let response = CreateAgentResponse { records_created: 1 };
        Json(response)
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
