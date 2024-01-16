use actix_web::{web, App, HttpServer};
use diesel::{prelude::*, r2d2};
use dotenvy::dotenv;
use std::env;
use std::io;
mod query;
use actix_web::middleware::Logger;
use env_logger::Env;
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // connect to SQLite DB

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    // start HTTP server on port 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api", web::post().to(query::query_data))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
