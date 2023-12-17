use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct CreateAgentResponse {
    pub records_created: u8,
}
