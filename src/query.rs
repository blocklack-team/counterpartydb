use actix_web::{error, web, HttpResponse, Responder};
use counterpartydb::balances::*;
use counterpartydb::bets::*;
use counterpartydb::blocks::*;
use counterpartydb::burns::*;
use counterpartydb::db::*;
use counterpartydb::debits::*;
use counterpartydb::dispensers::*;
use counterpartydb::dispenses::*;
use counterpartydb::issuances::*;
use counterpartydb::messages::*;
use counterpartydb::models::*;
use counterpartydb::sends::*;
use diesel::prelude::*;
use serde::Deserialize;
#[derive(Deserialize)]
enum QueryResult {
    Balances(Vec<Balance>),
    Blocks(Vec<Block>),
    Dispensers(Vec<Dispenser>),
    Debits(Vec<Debits>),
    Burn(Vec<Burn>),
    Issuances(Vec<Issuance>),
    Dispenses(Vec<Dispense>),
    Messages(Vec<Message>),
    Sends(Vec<Send>),
    Bets(Vec<Bet>),
}

fn _query_data(
    conn: &mut SqliteConnection,
    query_data: QueryData,
) -> Result<Option<QueryResult>, DbError> {
    let method = query_data.method;
    let filterop = query_data.filter_op;
    let filters = query_data.filters;
    let order = query_data.order;
    let order_by = query_data.order_by;
    match method.as_str() {
        "get_balances" => {
            let balances = get_balances(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Balances(balances)));
        }
        "get_blocks" => {
            let blocks = get_blocks(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Blocks(blocks)));
        }
        "get_dispensers" => {
            let dispensers = get_dispensers(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Dispensers(dispensers)));
        }
        "get_debits" => {
            let debits = get_debits(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Debits(debits)));
        }
        "get_burns" => {
            let burns = get_burns(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Burn(burns)));
        }
        "get_issuances" => {
            let issuances = get_issuances(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Issuances(issuances)));
        }
        "get_dispenses" => {
            let dispenses = get_dispenses(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Dispenses(dispenses)));
        }
        "get_messages" => {
            let messages = get_messages(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Messages(messages)));
        }
        "get_sends" => {
            let sends = get_sends(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Sends(sends)));
        }
        "get_bets" => {
            let bets = get_bets(
                conn,
                filters,
                query_data.limit,
                query_data.offset,
                filterop,
                order,
                order_by,
            )?;
            return Ok(Some(QueryResult::Bets(bets)));
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
        Some(QueryResult::Dispensers(dispensers)) => Ok(HttpResponse::Ok().json(dispensers)),
        Some(QueryResult::Debits(debits)) => Ok(HttpResponse::Ok().json(debits)),
        Some(QueryResult::Burn(burns)) => Ok(HttpResponse::Ok().json(burns)),
        Some(QueryResult::Issuances(issuances)) => Ok(HttpResponse::Ok().json(issuances)),
        Some(QueryResult::Dispenses(dispenses)) => Ok(HttpResponse::Ok().json(dispenses)),
        Some(QueryResult::Messages(messages)) => Ok(HttpResponse::Ok().json(messages)),
        Some(QueryResult::Sends(sends)) => Ok(HttpResponse::Ok().json(sends)),
        Some(QueryResult::Bets(bets)) => Ok(HttpResponse::Ok().json(bets)),
        //TODO: ADD more results
        _ => Ok(HttpResponse::NotFound().finish()),
    }
}
