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
        match field {
            "address" => {
                if let FilterValue::String(s) = value {
                    query = match filter.op {
                        Operator::Equal => query.filter(address.eq(s)),
                        Operator::NotEqual => query.filter(address.ne(s)),
                        _ => query,
                    }
                }
            }
            "quantity" => {
                if let FilterValue::Integer32(i) = value {
                    query = match filter.op {
                        Operator::GreaterThan => query.filter(quantity.gt(i)),
                        Operator::LessThan => query.filter(quantity.lt(i)),
                        Operator::Equal => query.filter(quantity.eq(i)),
                        _ => query,
                    }
                }
            }
            "asset" => {
                if let FilterValue::String(s) = value {
                    query = match filter.op {
                        Operator::Equal => query.filter(asset.eq(s)),
                        Operator::NotEqual => query.filter(asset.ne(s)),
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
