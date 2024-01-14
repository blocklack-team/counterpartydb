use actix_web::{error, web, HttpResponse, Responder};

use counterpartydb::*;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Deserialize, Serialize)]
enum FilterValue {
    String(String),
    Integer(i32),
    Integer64(i64),
}
#[derive(Deserialize, Serialize)]
struct DynamicFilter {
    field: String,
    value: FilterValue,
    operator: String,
}

#[derive(Deserialize)]
pub struct QueryData {
    method: String,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
}

fn get_balances(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
) -> Result<Vec<models::Balance>, DbError> {
    use counterpartydb::schema::balances::dsl::*;
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
                if let FilterValue::Integer(i) = value {
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
    let result = query.load::<models::Balance>(conn)?;
    Ok(result)
}

fn get_blocks(
    conn: &mut SqliteConnection,
    filters: Vec<DynamicFilter>,
    limit: i64,
    offset: i64,
) -> Result<Vec<models::Block>, DbError> {
    use counterpartydb::schema::blocks::dsl::*;
    let mut query = blocks.into_boxed();
    for filter in filters {
        let field = filter.field.as_str();
        let value = filter.value;
        let operator = filter.operator.as_str();
        match field {
            "block_index" => {
                if let FilterValue::Integer64(i) = value {
                    query = match operator {
                        ">" => query.filter(block_index.gt(i)),
                        "<" => query.filter(block_index.lt(i)),
                        "=" => query.filter(block_index.eq(i)),
                        _ => query,
                    }
                }
            }
            "block_hash" => {
                if let FilterValue::String(s) = value {
                    query = match operator {
                        "=" => query.filter(block_hash.eq(s)),
                        "!=" => query.filter(block_hash.ne(s)),
                        _ => query,
                    }
                }
            }
            "block_time" => {
                if let FilterValue::Integer(i) = value {
                    query = match operator {
                        ">" => query.filter(block_time.gt(i)),
                        "<" => query.filter(block_time.lt(i)),
                        "=" => query.filter(block_time.eq(i)),
                        _ => query,
                    }
                }
            }
            _ => {}
        }
    }
    query = query.limit(limit).offset(offset);
    let result = query.load::<models::Block>(conn)?;
    Ok(result)
}

#[derive(Deserialize)]
enum QueryResult {
    Balances(Vec<models::Balance>),
    Blocks(Vec<models::Block>),
}

fn _query_data(
    conn: &mut SqliteConnection,
    query_data: QueryData,
) -> Result<Option<QueryResult>, DbError> {
    let method = query_data.method;
    match method.as_str() {
        "get_balances" => {
            let filters = query_data.filters;
            let balances = get_balances(conn, filters, query_data.limit, query_data.offset)?;
            return Ok(Some(QueryResult::Balances(balances)));
        }
        "get_blocks" => {
            let filters = query_data.filters;
            let blocks = get_blocks(conn, filters, query_data.limit, query_data.offset)?;
            return Ok(Some(QueryResult::Blocks(blocks)));
        }
        _ => {}
    }
    Ok(None)
}

pub async fn query_data(
    pool: web::Data<DbPool>,
    query_dat: web::Json<QueryData>,
) -> actix_web::Result<impl Responder> {
    let query = query_dat.into_inner();

    let asset_in_db = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        _query_data(&mut conn, query)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    match asset_in_db {
        Some(QueryResult::Balances(balances)) => Ok(HttpResponse::Ok().json(balances)),
        Some(QueryResult::Blocks(blocks)) => Ok(HttpResponse::Ok().json(blocks)),
        //TODO: ADD more results
        _ => Ok(HttpResponse::NotFound().finish()),
    }
}
