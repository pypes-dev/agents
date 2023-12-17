use serde::Deserialize;

// The query parameters for all generic read requests
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
