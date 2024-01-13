use counterpartydb::models::*;
use diesel::prelude::*;
use counterpartydb::*;
use actix_web::{error,  web,  HttpResponse,  Responder};
use diesel::r2d2;
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

type DbError = Box<dyn std::error::Error + Send + Sync>;

fn _get_balance(
    conn: &mut SqliteConnection,
    address_owner: String) -> Result<Option<Vec<models::Balance>>, DbError> {
    use counterpartydb::schema::balances::dsl::*;
    let result = balances
        .filter(address.eq(address_owner))
        .load::<Balance>(conn)
        .optional()?;

    Ok(result)
} 

pub async fn get_balance(
    pool: web::Data<DbPool>,
    address: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let (addres_owner,) = address.into_inner();

    let asset_in_db = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        _get_balance(&mut conn, addres_owner)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(asset_in_db))
}