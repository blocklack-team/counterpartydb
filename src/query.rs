use actix_web::{error, web, HttpResponse, Responder};
use counterpartydb::addresses::*;
use counterpartydb::balances::*;
use counterpartydb::bets::*;
use counterpartydb::blocks::*;
use counterpartydb::burns::*;
use counterpartydb::credits::*;
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
    Addresses(Vec<Address>),
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
    Credits(Vec<Credit>),
}

fn _query_data(
    conn: &mut SqliteConnection,
    query_data: QueryData,
) -> Result<Option<QueryResult>, DbError> {
    let q = query_data.clone();
    let method = query_data.method;
    match method.as_str() {
        "get_balances" => {
            let balances = get_balances(conn, q)?;
            return Ok(Some(QueryResult::Balances(balances)));
        }
        "get_blocks" => {
            let blocks = get_blocks(conn, q)?;
            return Ok(Some(QueryResult::Blocks(blocks)));
        }
        "get_dispensers" => {
            let dispensers = get_dispensers(conn, q)?;
            return Ok(Some(QueryResult::Dispensers(dispensers)));
        }
        "get_debits" => {
            let debits = get_debits(conn, q)?;
            return Ok(Some(QueryResult::Debits(debits)));
        }
        "get_burns" => {
            let burns = get_burns(conn, q)?;
            return Ok(Some(QueryResult::Burn(burns)));
        }
        "get_issuances" => {
            let issuances = get_issuances(conn, q)?;
            return Ok(Some(QueryResult::Issuances(issuances)));
        }
        "get_dispenses" => {
            let dispenses = get_dispenses(conn, q)?;
            return Ok(Some(QueryResult::Dispenses(dispenses)));
        }
        "get_messages" => {
            let messages = get_messages(conn, q)?;
            return Ok(Some(QueryResult::Messages(messages)));
        }
        "get_sends" => {
            let sends = get_sends(conn, q)?;
            return Ok(Some(QueryResult::Sends(sends)));
        }
        "get_bets" => {
            let bets = get_bets(conn, q)?;
            return Ok(Some(QueryResult::Bets(bets)));
        }
        "get_addresses" => {
            let addresses = get_addresses(conn, q)?;
            return Ok(Some(QueryResult::Addresses(addresses)));
        }
        "get_credits" => {
            let credits = get_credits(conn, q)?;
            return Ok(Some(QueryResult::Credits(credits)));
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
        Some(QueryResult::Addresses(addresses)) => Ok(HttpResponse::Ok().json(addresses)),
        Some(QueryResult::Credits(credits)) => Ok(HttpResponse::Ok().json(credits)),
        //TODO: ADD more results
        _ => Ok(HttpResponse::NotFound().finish()),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use super::*;
    use diesel::r2d2;
    use dotenvy::dotenv;
    use env_logger::Env;
    use std::env;
    #[actix_web::test]
    async fn test_query_data() {
        dotenv().ok();
        // connect to SQLite DB

        env_logger::init_from_env(Env::default().default_filter_or("info"));
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("database URL should be valid path to SQLite DB file");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api", web::post().to(query_data)),
        )
        .await;
        let requests = r#"
        {
            "method": "get_balances",
            "filters": [
                {
                    "field": "address",
                    "op": "=",
                    "value": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
                }
            ],
            "limit": 10,
            "offset": 0
        }"#;
        let requests: QueryData = serde_json::from_str(requests).unwrap();
        println!("requests: {:?}", requests);
        let req = test::TestRequest::post()
            .uri("/api")
            .set_json(&requests)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
