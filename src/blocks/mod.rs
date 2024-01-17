use crate::db::*;
use crate::models::get_all_columns;
use crate::models::Block;
use diesel::{prelude::*, sql_query};

pub fn get_blocks(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
    filterop: FilterOp,
    order: Order,
    order_by: String,
) -> Result<Vec<Block>, DbError> {
    let columns = get_all_columns("blocks");
    let query_string = generate_sql_query(
        filters, limit, offset, filterop, order, order_by, &columns, "blocks",
    );
    if query_string.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        )));
    }
    let result = sql_query(&query_string.unwrap()).load::<Block>(conn);
    match result {
        Ok(r) => Ok(r),
        Err(_e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        ))),
    }
}
