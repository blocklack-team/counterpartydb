use crate::db::*;
use crate::models::get_all_columns;
use crate::models::Burn;
use diesel::{prelude::*, sql_query};

pub fn get_burns(conn: &mut SqliteConnection, query: QueryData) -> Result<Vec<Burn>, DbError> {
    let (filters, limit, offset, filterop, order, order_by) = query.tuple();
    let columns = get_all_columns("burns");
    let query_string = generate_sql_query(
        filters, limit, offset, filterop, order, order_by, &columns, "burns",
    );
    if query_string.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        )));
    }
    let result = sql_query(&query_string.unwrap()).load::<Burn>(conn);
    match result {
        Ok(r) => Ok(r),
        Err(_e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to generate SQL query.",
        ))),
    }
}
