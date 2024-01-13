use counterpartydb::models::*;
use diesel::prelude::*;
use counterpartydb::*;
use actix_web::{error,  web,  HttpResponse,  Responder};
use diesel::r2d2;
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

type DbError = Box<dyn std::error::Error + Send + Sync>;


fn _get_block_by_index(
    conn: &mut SqliteConnection,
    index: i64) -> Result<Option<Vec<models::Block>>, DbError> {
    use counterpartydb::schema::blocks::dsl::*;
    let result = blocks
        .filter(block_index.eq(index))
        .load::<Block>(conn)
        .optional()?;
    Ok(result)
} 

fn _get_block_by_hash(
    conn: &mut SqliteConnection,
    hash: String) -> Result<Option<Vec<models::Block>>, DbError> {
    use counterpartydb::schema::blocks::dsl::*;
    let result = blocks
        .filter(block_hash.eq(hash))
        .load::<Block>(conn)
        .optional()?;
    Ok(result)
} 

pub async fn get_block_by_index(
    pool: web::Data<DbPool>,
    block_index: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (block_index,) = block_index.into_inner();

    let asset_in_db = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        _get_block_by_index(&mut conn, block_index)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(asset_in_db))
}

pub async fn get_block_by_hash(
    pool: web::Data<DbPool>,
    block_hash: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let (hash,) = block_hash.into_inner();

    let asset_in_db = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        _get_block_by_hash(&mut conn, hash)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(asset_in_db))
}