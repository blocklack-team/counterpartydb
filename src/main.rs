extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::{prelude::*, r2d2};
use dotenvy::dotenv;
use std::env;
use std::io;
mod assets;
mod balances;
mod blocks;
mod query;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // connect to SQLite DB
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port_str = env::var("PORT").expect("PORT must be set");
    let port: u16 = port_str.parse().expect("PORT must be an integer");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    // start HTTP server on port 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api", web::post().to(query::query_data))
            .route("/asset_info/{asset_name}", web::get().to(assets::get_asset))
            .route("/balance/{address}", web::get().to(balances::get_balance))
            .route(
                "/block/{block_index}",
                web::get().to(blocks::get_block_by_index),
            )
            .route(
                "/block_hash/{block_hash}",
                web::get().to(blocks::get_block_by_hash),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
