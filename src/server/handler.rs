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
