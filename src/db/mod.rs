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

#[derive(Deserialize, Serialize, Debug)]
pub enum Order {
    #[serde(rename = "ASC")]
    ASC,
    #[serde(rename = "DESC")]
    DESC,
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
    #[serde(default = "default_order")]
    pub order: Order,
    #[serde(default = "default_order_by")]
    pub order_by: String,
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

fn default_order() -> Order {
    Order::ASC
}

fn default_order_by() -> String {
    String::new()
}

fn generate_filter_clause(
    field: &str,
    value: FilterValue,
    op: Operator,
    columns: &Vec<&str>,
) -> Option<String> {
    let column_name;

    if columns.contains(&field) {
        column_name = field.to_string();
    } else {
        return Some("".to_string());
    }

    let sql_operator = op.to_string();

    let value_str = match value {
        FilterValue::Integer64(i) => i.to_string(),
        FilterValue::String(s) => format!("'{}'", s.escape_default()),
        FilterValue::Integer32(i) => i.to_string(),
        FilterValue::Float32(f) => f.to_string(),
        FilterValue::Float64(f) => f.to_string(),
    };

    Some(format!("{} {} {}", column_name, sql_operator, value_str))
}

pub fn generate_sql_query(
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
    filterop: FilterOp,
    order: Order,
    order_by: String,
    columns: &Vec<&str>,
    table_name: &str,
) -> Option<String> {
    let mut filter_clauses: Vec<String> = vec![];
    for filter in filters {
        let field = filter.field.as_str();
        let value = filter.value;
        let op = filter.op;

        if let Some(filter_clause) = generate_filter_clause(field, value, op, columns) {
            filter_clauses.push(filter_clause);
        }
    }

    if filter_clauses.is_empty() {
        let limit_offset = format!("LIMIT {} OFFSET {}", limit, offset);
        return Some(format!("SELECT * FROM {} {}", table_name, limit_offset));
    }
    let filter_string = match filterop {
        FilterOp::And => filter_clauses.join(" AND "),
        FilterOp::Or => filter_clauses.join(" OR "),
    };
    let mut order_by_default = order_by.clone();
    if order_by.is_empty() == false {
        //check by order_by field is valid
        if columns.contains(&order_by.as_str()) == false {
            println!("Invalid order_by field: {}", order_by);
            return None;
        }
    } else {
        order_by_default = "quantity".to_string();
    }
    let order_clause = match order {
        Order::ASC => format!("ORDER BY {} ASC", order_by_default),
        Order::DESC => format!("ORDER BY {} DESC", order_by_default),
    };
    println!("order_clause: {}", order_clause);
    let limit_offset = format!("LIMIT {} OFFSET {}", limit, offset);

    Some(format!(
        "SELECT * FROM {} WHERE {} {} {}",
        table_name, filter_string, order_clause, limit_offset
    ))
}
