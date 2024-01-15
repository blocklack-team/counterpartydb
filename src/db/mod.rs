use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum FilterValue {
    String(String),
    Integer64(i64),
    Integer32(i32),
}
#[derive(Deserialize, Serialize)]
pub struct DynamicFilter {
    pub field: String,
    pub value: FilterValue,
    pub operator: String,
}

#[derive(Deserialize, Serialize)]
pub struct QueryData {
    pub method: String,
    pub filters: Vec<DynamicFilter>,
    pub limit: i64,
    pub offset: i64,
}
