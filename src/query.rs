use actix_web::{error, web, HttpResponse, Responder};
use counterpartydb::balances::*;
use counterpartydb::blocks::*;
use counterpartydb::db::*;
use counterpartydb::models::*;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
enum QueryResult {
    Balances(Vec<Balance>),
    Blocks(Vec<Block>),
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
