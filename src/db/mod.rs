use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Deserialize, Serialize)]
pub enum Operator {
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "==")]
    StrictEqual,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
}
impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Equal => "=".to_string(),
            Operator::StrictEqual => "==".to_string(),
            Operator::NotEqual => "!=".to_string(),
            Operator::GreaterThan => ">".to_string(),
            Operator::LessThan => "<".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum FilterOp {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum FilterValue {
    String(String),
    Integer64(i64),
    Integer32(i32),
    Float64(f64),
    Float32(f32),
}
#[derive(Deserialize, Serialize)]
pub struct DynamicFilter {
    pub field: String,
    pub value: FilterValue,
    #[serde(default = "default_operator")]
    pub op: Operator,
}

#[derive(Deserialize, Serialize)]
pub struct QueryData {
    pub method: String,
    pub filters: Vec<DynamicFilter>,
    #[serde(default = "default_filter_op")]
    pub filter_op: FilterOp,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
    order_by: Option<String>,
}

fn default_limit() -> i64 {
    100
}
fn default_offset() -> i64 {
    0
}
fn default_filter_op() -> FilterOp {
    FilterOp::And
}
fn default_operator() -> Operator {
    Operator::Equal
}
