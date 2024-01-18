use crate::schema::*;
use diesel::prelude::*;
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = addresses)]
pub struct Address {
    pub address: String,
    pub options: Option<i32>,
    pub block_index: Option<i32>,
}

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
    quantity: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = blocks)]
pub struct Block {
    pub block_index: i32,
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
    pub give_quantity: Option<i64>,
    pub escrow_quantity: Option<i64>,
    pub satoshirate: Option<i64>,
    pub status: Option<i32>,
    pub give_remaining: Option<i32>,
    pub oracle_address: Option<String>,
    pub last_status_tx_hash: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = debits)]
pub struct Debits {
    pub block_index: i32,
    pub address: Option<String>,
    pub asset: Option<String>,
    pub quantity: Option<i64>,
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
    pub wager_quantity: i64,
    pub wager_remaining: i64,
    pub counterwager_quantity: i64,
    pub counterwager_remaining: i64,
    pub target_value: f32,
    pub leverage: i32,
    pub expiration: i32,
    pub expire_index: i32,
    pub fee_fraction_int: i64,
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
    pub initial_value: i64,
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
    pub fee_fraction_int: i64,
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
    pub fee: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = credits)]
pub struct Credit {
    pub block_index: i32,
    pub address: String,
    pub asset: Option<String>,
    pub quantity: Option<i32>,
    pub calling_function: Option<String>,
    pub event: Option<String>,
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
    pub quantity: Option<i64>,
    pub divisible: Option<bool>,
    pub source: Option<String>,
    pub issuer: Option<String>,
    pub transfer: Option<bool>,
    pub callable: Option<bool>,
    pub call_date: Option<i32>,
    pub call_price: Option<f32>,
    pub description: Option<String>,
    pub fee_paid: Option<i64>,
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
    pub quantity: Option<i64>,
    pub status: Option<String>,
    pub msg_index: Option<i32>,
    pub memo: Option<Vec<u8>>,
}

pub fn get_all_tables() -> Vec<String> {
    let tables = vec![
        "balances",
        "assets",
        "blocks",
        "dispensers",
        "debits",
        "broadcasts",
        "btcpays",
        "burns",
        "bets",
        "bet_matches",
        "bet_match_resolutions",
        "bet_match_expirations",
        "bet_expirations",
        "issuances",
        "dispenses",
        "messages",
        "sends",
    ];
    tables.iter().map(|s| s.to_string()).collect()
}

pub fn get_all_columns(table: &str) -> Vec<&str> {
    let columns = match table {
        "addresses" => vec!["address", "options", "block_index"],
        "balances" => vec!["address", "asset", "quantity"],
        "assets" => vec!["asset_id", "asset_name", "block_index", "asset_longname"],
        "credits" => vec![
            "block_index",
            "address",
            "asset",
            "quantity",
            "calling_function",
            "event",
        ],
        "blocks" => vec![
            "block_index",
            "block_hash",
            "block_time",
            "previous_block_hash",
            "difficulty",
            "ledger_hash",
            "txlist_hash",
            "messages_hash",
        ],
        "dispensers" => vec![
            "tx_index",
            "tx_hash",
            "block_index",
            "source",
            "asset",
            "give_quantity",
            "escrow_quantity",
            "satoshirate",
            "status",
            "give_remaining",
            "oracle_address",
            "last_status_tx_hash",
            "origin",
        ],
        "debits" => vec![
            "block_index",
            "address",
            "asset",
            "quantity",
            "action",
            "event",
        ],
        "broadcasts" => vec![
            "tx_index",
            "tx_hash",
            "block_index",
            "source",
            "timestamp",
            "value",
            "fee_fraction_int",
            "text",
            "locked",
            "status",
        ],
        "btcpays" => vec![
            "tx_index",
            "tx_hash",
            "block_index",
            "source",
            "destination",
            "btc_amount",
            "order_match_id",
            "status",
        ],
        "burns" => vec![
            "tx_index",
            "tx_hash",
            "block_index",
            "source",
            "burned",
            "earned",
            "status",
        ],
        "bets" => vec![
            "tx_index",
            "tx_hash",
            "block_index",
            "source",
            "feed_address",
            "bet_type",
            "deadline",
            "wager_quantity",
            "wager_remaining",
            "counterwager_quantity",
            "counterwager_remaining",
            "target_value",
            "leverage",
            "expiration",
            "expire_index",
            "fee_fraction_int",
            "status",
        ],
        "bet_matches" => vec![
            "id",
            "tx0_index",
            "tx0_hash",
            "tx0_address",
            "tx1_index",
            "tx1_hash",
            "tx1_address",
            "tx0_bet_type",
            "tx1_bet_type",
            "feed_address",
            "initial_value",
            "deadline",
            "target_value",
            "leverage",
            "forward_quantity",
            "backward_quantity",
            "tx0_block_index",
            "tx1_block_index",
            "block_index",
            "tx0_expiration",
            "tx1_expiration",
            "match_expire_index",
            "fee_fraction_int",
            "status",
        ],
        "bet_match_resolutions" => vec![
            "bet_match",
            "bet_match_type_id",
            "block_index",
            "winner",
            "settled",
            "bull_credit",
            "bear_credit",
            "escrow_less_fee",
            "fee",
        ],
        "bet_match_expirations" => vec!["bet_match", "tx0_address", "tx1_address", "block_index"],
        "bet_expirations" => vec!["bet_index", "bet_hash", "source", "block_index"],
        "issuances" => vec![
            "tx_index",
            "tx_hash",
            "msg_index",
            "block_index",
            "asset",
            "quantity",
            "divisible",
            "source",
            "issuer",
            "transfer",
            "callable",
            "call_date",
            "call_price",
            "description",
            "fee_paid",
            "locked",
            "status",
            "asset_longname",
            "reset",
        ],
        "dispenses" => vec![
            "tx_index",
            "dispense_index",
            "tx_hash",
            "block_index",
            "source",
            "destination",
            "asset",
            "dispense_quantity",
            "dispenser_tx_hash",
        ],
        "messages" => vec![
            "message_index",
            "block_index",
            "command",
            "category",
            "bindings",
            "timestamp",
        ],
        "sends" => vec![
            "tx_index",
            "tx_hash",
            "block_index",
            "source",
            "destination",
            "asset",
            "quantity",
            "status",
            "msg_index",
            "memo",
        ],
        _ => vec![],
    };
    columns
}
