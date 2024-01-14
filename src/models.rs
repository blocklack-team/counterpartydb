use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = assets)]
pub struct Asset {
    asset_id: String,
    asset_name: Option<String>,
    block_index: Option<i32>,
    asset_longname: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = balances)]
pub struct Balance {
    pub address: String,
    pub asset: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
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
