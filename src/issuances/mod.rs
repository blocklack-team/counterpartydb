use crate::db::*;
use crate::models::get_all_columns;
use crate::models::Issuance;
use diesel::{prelude::*, sql_query};
pub fn get_issuances(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
    filterop: FilterOp,
    order: Order,
    order_by: String,
) -> Result<Vec<Issuance>, DbError> {
    let columns = get_all_columns("issuances");
    let query_string = generate_sql_query(
        filters,
        limit,
        offset,
        filterop,
        order,
        order_by,
        &columns,
        "issuances",
    );
    if query_string.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        )));
    }
    let result = sql_query(&query_string.unwrap()).load::<Issuance>(conn);
    match result {
        Ok(r) => Ok(r),
        Err(_e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        ))),
    }
}
