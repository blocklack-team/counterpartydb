use crate::db::*;
use crate::models::Balance;
use crate::schema::balances::dsl::*;
use diesel::prelude::*;

pub fn get_balances(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Balance>, DbError> {
    let mut query = balances.into_boxed();
    for filter in filters {
        let field = filter.field.as_str();
        let value = filter.value;
        let operator = filter.operator.as_str();
        match field {
            "address" => {
                if let FilterValue::String(s) = value {
                    query = match operator {
                        "=" => query.filter(address.eq(s)),
                        "!=" => query.filter(address.ne(s)),
                        _ => query,
                    }
                }
            }
            "quantity" => {
                if let FilterValue::Integer32(i) = value {
                    query = match operator {
                        ">" => query.filter(quantity.gt(i)),
                        "<" => query.filter(quantity.lt(i)),
                        "=" => query.filter(quantity.eq(i)),
                        _ => query,
                    }
                }
            }
            "asset" => {
                if let FilterValue::String(s) = value {
                    query = match operator {
                        "=" => query.filter(asset.eq(s)),
                        "!=" => query.filter(asset.ne(s)),
                        _ => query,
                    }
                }
            }
            _ => {}
        }
    }
    query = query.limit(limit).offset(offset);
    let result = query.load::<Balance>(conn)?;
    Ok(result)
}
