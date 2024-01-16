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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = broadcasts)]
pub struct Broadcast {
    pub tx_index: i32,
    pub tx_hash: String,
    pub block_index: i32,
    pub source: String,
    pub timestamp: i32,
    pub value: f32,
    pub fee_fraction_int: i32,
    pub text: String,
    pub locked: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = btcpays)]
pub struct Btcpay {
    pub tx_index: i32,
    pub tx_hash: String,
    pub block_index: i32,
    pub source: String,
    pub destination: String,
    pub btc_amount: i32,
    pub order_match_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = burns)]
pub struct Burn {
    pub tx_index: i32,
    pub tx_hash: String,
    pub block_index: i32,
    pub source: String,
    pub burned: i32,
    pub earned: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = bets)]
pub struct Bet {
    pub tx_index: i32,
    pub tx_hash: String,
    pub block_index: i32,
    pub source: String,
    pub feed_address: String,
    pub bet_type: i32,
    pub deadline: i32,
    pub wager_quantity: i32,
    pub wager_remaining: i32,
    pub counterwager_quantity: i32,
    pub counterwager_remaining: i32,
    pub target_value: f32,
    pub leverage: i32,
    pub expiration: i32,
    pub expire_index: i32,
    pub fee_fraction_int: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = bet_matches)]
pub struct BetMatch {
    pub id: String,
    pub tx0_index: i32,
    pub tx0_hash: String,
    pub tx0_address: String,
    pub tx1_index: i32,
    pub tx1_hash: String,
    pub tx1_address: String,
    pub tx0_bet_type: i32,
    pub tx1_bet_type: i32,
    pub feed_address: String,
    pub initial_value: i32,
    pub deadline: i32,
    pub target_value: f32,
    pub leverage: i32,
    pub forward_quantity: i32,
    pub backward_quantity: i32,
    pub tx0_block_index: i32,
    pub tx1_block_index: i32,
    pub block_index: i32,
    pub tx0_expiration: i32,
    pub tx1_expiration: i32,
    pub match_expire_index: i32,
    pub fee_fraction_int: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = bet_match_resolutions)]
pub struct BetMatchResolution {
    pub bet_match: String,
    pub bet_match_type_id: i32,
    pub block_index: i32,
    pub winner: String,
    pub settled: bool,
    pub bull_credit: i32,
    pub bear_credit: i32,
    pub escrow_less_fee: i32,
    pub fee: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = bet_match_expirations)]
pub struct BetMatchExpiration {
    pub bet_match: String,
    pub tx0_address: String,
    pub tx1_address: String,
    pub block_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = bet_expirations)]
pub struct BetExpiration {
    pub bet_index: i32,
    pub bet_hash: String,
    pub source: String,
    pub block_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = issuances)]
pub struct Issuance {
    pub tx_index: Option<i32>,
    pub tx_hash: Option<String>,
    pub msg_index: Option<i32>,
    pub block_index: Option<i32>,
    pub asset: Option<String>,
    pub quantity: Option<i32>,
    pub divisible: Option<bool>,
    pub source: Option<String>,
    pub issuer: Option<String>,
    pub transfer: Option<bool>,
    pub callable: Option<bool>,
    pub call_date: Option<i32>,
    pub call_price: Option<f32>,
    pub description: Option<String>,
    pub fee_paid: Option<i32>,
    pub locked: Option<bool>,
    pub status: Option<String>,
    pub asset_longname: Option<String>,
    pub reset: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = dispenses)]
pub struct Dispense {
    pub tx_index: i32,
    pub dispense_index: Option<i32>,
    pub tx_hash: Option<String>,
    pub block_index: Option<i32>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub asset: Option<String>,
    pub dispense_quantity: Option<i32>,
    pub dispenser_tx_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = messages)]
pub struct Message {
    pub message_index: i32,
    pub block_index: Option<i32>,
    pub command: Option<String>,
    pub category: Option<String>,
    pub bindings: Option<String>,
    pub timestamp: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = sends)]
pub struct Send {
    pub tx_index: i32,
    pub tx_hash: Option<String>,
    pub block_index: Option<i32>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub asset: Option<String>,
    pub quantity: Option<i32>,
    pub status: Option<String>,
    pub msg_index: Option<i32>,
    pub memo: Option<Vec<u8>>, // BinaryField is represented as Vec<u8> in Rust
}
