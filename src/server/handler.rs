pub mod agents {
    use crate::agent::agent::Agent;
    use crate::server::requests;
    use crate::server::responses;
    use axum::{
        extract::{Query, State},
        response::IntoResponse,
        Json,
    };
    use pickledb::PickleDb;
    use serde::Deserialize;
    use std::sync::{Arc, RwLock};
    type Db = Arc<RwLock<PickleDb>>;
    pub async fn agents_index(
        _pagination: Option<Query<requests::Pagination>>,
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
        let response = responses::CreateAgentResponse { records_created: 1 };
        Json(response)
    }
}
