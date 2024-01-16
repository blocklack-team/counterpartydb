use crate::db::*;
use crate::models::Debits;
use diesel::{prelude::*, sql_query};

fn generate_filter_clause(field: &str, value: FilterValue, op: Operator) -> Option<String> {
    println!("Field: {:?}", field);
    let column_name = match field {
        "block_index" => "block_index",
        "address" => "address",
        "asset" => "asset",
        "quantity" => "quantity",
        "action" => "action",
        "event" => "event",
        _ => return Some("".to_string()), // Salta filtros no reconocidos
    };
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

pub fn generate_sql_query(filters: Vec<DynamicFilter>, limit: i64, offset: i64) -> Option<String> {
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

    let filter_string = filter_clauses.join(" AND ");
    let limit_offset = format!("LIMIT {} OFFSET {}", limit, offset);

    Some(format!(
        "SELECT * FROM debits WHERE {} {}",
        filter_string, limit_offset
    ))
}

pub fn get_debits(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Debits>, DbError> {
    let query_string = generate_sql_query(filters, limit, offset);
    println!("Query string: {:?}", query_string);
    let result = sql_query(&query_string.unwrap()).load::<Debits>(conn);
    match result {
        Ok(r) => Ok(r),
        Err(_e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        ))),
    }
}
