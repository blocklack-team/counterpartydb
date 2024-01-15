use crate::schema::*;
use diesel::prelude::*;
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};
/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = assets)]
pub struct Asset {
    asset_id: String,
    asset_name: Option<String>,
    block_index: Option<i32>,
    asset_longname: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = balances)]
pub struct Balance {
    address: String,
    asset: Option<String>,
    quantity: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = blocks)]
pub struct Block {
    pub block_index: i64, // Asumiendo que AutoField se mapea a i32 en Rust
    pub block_hash: Option<String>,
    pub block_time: Option<i32>,
    pub previous_block_hash: Option<String>,
    pub difficulty: Option<i32>,
    pub ledger_hash: Option<String>,
    pub txlist_hash: Option<String>,
    pub messages_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = dispensers)]
pub struct Dispenser {
    pub tx_index: i32,
    pub tx_hash: Option<String>,
    pub block_index: Option<i32>,
    pub source: Option<String>,
    pub asset: Option<String>,
    pub give_quantity: Option<i32>,
    pub escrow_quantity: Option<i32>,
    pub satoshirate: Option<i32>,
    pub status: Option<i32>,
    pub give_remaining: Option<i32>,
    pub oracle_address: Option<String>,
    pub last_status_tx_hash: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = debits)]
pub struct Debits {
    pub block_index: i32, // Ajusta el tipo de dato según corresponda
    pub address: Option<String>,
    pub asset: Option<String>,
    pub quantity: Option<i32>, // Ajusta el tipo de dato según corresponda
    pub action: Option<String>,
    pub event: Option<String>,
}
