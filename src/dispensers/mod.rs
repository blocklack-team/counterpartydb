use crate::db::*;
use crate::models::get_all_columns;
use crate::models::Dispenser;
use diesel::{prelude::*, sql_query};

pub fn get_dispensers(
    conn: &mut SqliteConnection,
    query: QueryData,
) -> Result<Vec<Dispenser>, DbError> {
    let (filters, limit, offset, filterop, order, order_by) = query.tuple();
    let columns = get_all_columns("dispensers");
    let query_string = generate_sql_query(
        filters,
        limit,
        offset,
        filterop,
        order,
        order_by,
        &columns,
        "dispensers",
    );
    if query_string.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        )));
    }
    let result = sql_query(&query_string.unwrap()).load::<Dispenser>(conn);
    match result {
        Ok(r) => Ok(r),
        Err(_e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        ))),
    }
}
