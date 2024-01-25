use crate::db::*;
use crate::models::{get_all_columns, Balance};
use diesel::{prelude::*, sql_query};

pub fn get_balances(
    conn: &mut SqliteConnection,
    query: QueryData,
) -> Result<Vec<Balance>, DbError> {
    let (filters, limit, offset, filterop, order, order_by) = query.tuple();

    let columns = get_all_columns("balances");
    let query_string = generate_sql_query(
        filters, limit, offset, filterop, order, order_by, &columns, "balances",
    );
    if query_string.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        )));
    }
    println!("Query string: {:?}", query_string);
    let result = sql_query(&query_string.unwrap()).load::<Balance>(conn);
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Unable to execute SQL query: {:?}", e),
        ))),
    }
}
