use crate::db::*;
use crate::models::Block;
use crate::schema::blocks::dsl::*;
use diesel::prelude::*;

pub fn get_blocks(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Block>, DbError> {
    let mut query = blocks.into_boxed();
    for filter in filters {
        let field = filter.field.as_str();
        let value = filter.value;
        match field {
            "block_index" => {
                if let FilterValue::Integer64(i) = value {
                    query = match filter.op {
                        Operator::GreaterThan => query.filter(block_index.gt(i)),
                        Operator::LessThan => query.filter(block_index.lt(i)),
                        Operator::Equal => query.filter(block_index.eq(i)),
                        _ => query,
                    }
                }
            }
            "block_hash" => {
                if let FilterValue::String(s) = value {
                    query = match filter.op {
                        Operator::Equal => query.filter(block_hash.eq(s)),
                        Operator::NotEqual => query.filter(block_hash.ne(s)),
                        _ => query,
                    }
                }
            }
            "block_time" => {
                if let FilterValue::Integer32(i) = value {
                    query = match filter.op {
                        Operator::GreaterThan => query.filter(block_time.gt(i)),
                        Operator::LessThan => query.filter(block_time.lt(i)),
                        Operator::Equal => query.filter(block_time.eq(i)),
                        _ => query,
                    }
                }
            }
            _ => {}
        }
    }
    query = query.limit(limit).offset(offset);
    let result = query.load::<Block>(conn)?;
    Ok(result)
}
