use crate::bitcoin_utils::{deserialize_rawtx, request_tx};
use crate::counterparty::CounterPartyTransaction;
use actix_web::{
    web::{self},
    HttpRequest, Responder,
};
use serde::*;

use super::CounterPartyMessage;
#[derive(Serialize, Deserialize, Debug)]
pub struct Rawtx {
    rawtx: String,
}

pub async fn get_info_tx(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let tx_hash = req.match_info().get("tx_hash").unwrap();
    let result = request_tx(tx_hash).await;
    Ok(web::Json(serde_json::to_value(result).unwrap()))
}

pub async fn get_info_rawtx(req: web::Json<Rawtx>) -> actix_web::Result<impl Responder> {
    let rawtx = req.into_inner();
    let result = deserialize_rawtx(&rawtx.rawtx).await;
    //return Ok(web::Json(serde_json::to_value(result).unwrap()));
    match result {
        Some(r) => {
            let tx = CounterPartyTransaction { transaction: r };
            let enchance = tx.get_tx_decoded();
            match enchance {
                Some(e) => match e {
                    CounterPartyMessage::EnchanceSend(enchance_send) => {
                        Ok(web::Json(enchance_send))
                    }
                },
                None => Err(actix_web::error::ErrorBadRequest(
                    "Invalid counterparty tx data",
                )),
            }
        }
        None => Err(actix_web::error::ErrorBadRequest("Invalid rawtx")),
    }
}
