use crate::db::*;
use crate::models::Burn;
use diesel::{prelude::*, sql_query};
fn generate_filter_clause(field: &str, value: FilterValue, op: Operator) -> Option<String> {
    let column_name = match field {
        "tx_index" => "tx_index",
        "tx_hash" => "tx_hash",
        "block_index" => "block_index",
        "source" => "source",
        "burned" => "burned",
        "earned" => "earned",
        "status" => "status",
        _ => return Some("".to_string()), // Salta filtros no reconocidos
    };
    let sql_operator = op.to_string();

    let value_str = match value {
        FilterValue::Integer64(i) => i.to_string(),
        FilterValue::String(s) => format!("'{}'", s.to_uppercase()),
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
) -> Option<String> {
    let mut filter_clauses: Vec<String> = vec![];

    for filter in filters {
        let field = filter.field.as_str();
        let value = filter.value;
        let op = filter.op;

        if let Some(filter_clause) = generate_filter_clause(field, value, op) {
            filter_clauses.push(filter_clause);
        }
    }

    if filter_clauses.is_empty() {
        return None;
    }

    let filter_string = match filterop {
        FilterOp::And => filter_clauses.join(" AND "),
        FilterOp::Or => filter_clauses.join(" OR "),
    };
    let limit_offset = format!("LIMIT {} OFFSET {}", limit, offset);

    Some(format!(
        "SELECT * FROM burns WHERE {} {}",
        filter_string, limit_offset
    ))
}

pub fn get_burns(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
    filterop: FilterOp,
) -> Result<Vec<Burn>, DbError> {
    let query_string = generate_sql_query(filters, limit, offset, filterop);
    let result = sql_query(&query_string.unwrap()).load::<Burn>(conn);
    match result {
        Ok(r) => Ok(r),
        Err(_e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        ))),
    }
}
