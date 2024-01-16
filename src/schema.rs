// @generated automatically by Diesel CLI.
use diesel::table;
table! {
    assets (asset_id) {
        asset_id -> Text,
        asset_name -> Nullable<Text>,
        block_index -> Nullable<Integer>,
        asset_longname -> Nullable<Text>,
    }
}

table! {
    balances (address) {
        address -> Text,
        asset -> Nullable<Text>,
        quantity -> Nullable<Integer>,
    }
}

table! {
    blocks (block_index) {
        block_index -> BigInt,
        block_hash -> Nullable<Text>,
        block_time -> Nullable<Integer>,
        previous_block_hash -> Nullable<Text>,
        difficulty -> Nullable<Integer>,
        ledger_hash -> Nullable<Text>,
        txlist_hash -> Nullable<Text>,
        messages_hash -> Nullable<Text>,
    }
}
table! {
    dispensers (tx_index) {
        tx_index -> Integer,
        tx_hash -> Nullable<Text>,
        block_index -> Nullable<Integer>,
        source -> Nullable<Text>,
        asset -> Nullable<Text>,
        give_quantity -> Nullable<Integer>,
        escrow_quantity -> Nullable<Integer>,
        satoshirate -> Nullable<Integer>,
        status -> Nullable<Integer>,
        give_remaining -> Nullable<Integer>,
        oracle_address -> Nullable<Text>,
        last_status_tx_hash -> Nullable<Text>,
        origin -> Nullable<Text>,
    }
}

table! {
    debits (block_index) {
        block_index -> Integer,
        address -> Nullable<Text>,
        asset -> Nullable<Text>,
        quantity -> Nullable<Integer>,
        action -> Nullable<Text>,
        event -> Nullable<Text>,
    }
}

table! {
    broadcasts (tx_index) {
        tx_index -> Integer,
        tx_hash -> Text,
        block_index -> Integer,
        source -> Text,
        timestamp -> Integer,
        value -> Float,
        fee_fraction_int -> Integer,
        text -> Text,
        locked -> Bool,
        status -> Text,
    }
}

table! {
    btcpays (tx_index) {
        tx_index -> Integer,
        tx_hash -> Text,
        block_index -> Integer,
        source -> Text,
        destination -> Text,
        btc_amount -> Integer,
        order_match_id -> Text,
        status -> Text,
    }
}

table! {
    burns (tx_index) {
        tx_index -> Integer,
        tx_hash -> Text,
        block_index -> Integer,
        source -> Text,
        burned -> Integer,
        earned -> Integer,
        status -> Text,
    }
}

table! {
    bets (tx_index) {
        tx_index -> Integer,
        tx_hash -> Text,
        block_index -> Integer,
        source -> Text,
        feed_address -> Text,
        bet_type -> Integer,
        deadline -> Integer,
        wager_quantity -> Integer,
        wager_remaining -> Integer,
        counterwager_quantity -> Integer,
        counterwager_remaining -> Integer,
        target_value -> Float,
        leverage -> Integer,
        expiration -> Integer,
        expire_index -> Integer,
        fee_fraction_int -> Integer,
        status -> Text,
    }
}

table! {
    bet_matches (id) {
        id -> Text,
        tx0_index -> Integer,
        tx0_hash -> Text,
        tx0_address -> Text,
        tx1_index -> Integer,
        tx1_hash -> Text,
        tx1_address -> Text,
        tx0_bet_type -> Integer,
        tx1_bet_type -> Integer,
        feed_address -> Text,
        initial_value -> Integer,
        deadline -> Integer,
        target_value -> Float,
        leverage -> Integer,
        forward_quantity -> Integer,
        backward_quantity -> Integer,
        tx0_block_index -> Integer,
        tx1_block_index -> Integer,
        block_index -> Integer,
        tx0_expiration -> Integer,
        tx1_expiration -> Integer,
        match_expire_index -> Integer,
        fee_fraction_int -> Integer,
        status -> Text,
    }
}

table! {
    bet_match_resolutions (bet_match) {
        bet_match -> Text,
        bet_match_type_id -> Integer,
        block_index -> Integer,
        winner -> Text,
        settled -> Bool,
        bull_credit -> Integer,
        bear_credit -> Integer,
        escrow_less_fee -> Integer,
        fee -> Integer,
    }
}

table! {
    bet_match_expirations (bet_match) {
        bet_match -> Text,
        tx0_address -> Text,
        tx1_address -> Text,
        block_index -> Integer,
    }
}

table! {
    bet_expirations (bet_index) {
        bet_index -> Integer,
        bet_hash -> Text,
        source -> Text,
        block_index -> Integer,
    }
}

table! {
    issuances (tx_index) {
        tx_index -> Nullable<Integer>,
        tx_hash -> Nullable<Text>,
        msg_index -> Nullable<Integer>,
        block_index -> Nullable<Integer>,
        asset -> Nullable<Text>,
        quantity -> Nullable<Integer>,
        divisible -> Nullable<Bool>,
        source -> Nullable<Text>,
        issuer -> Nullable<Text>,
        transfer -> Nullable<Bool>,
        callable -> Nullable<Bool>,
        call_date -> Nullable<Integer>,
        call_price -> Nullable<Float>,
        description -> Nullable<Text>,
        fee_paid -> Nullable<Integer>,
        locked -> Nullable<Bool>,
        status -> Nullable<Text>,
        asset_longname -> Nullable<Text>,
        reset -> Nullable<Bool>,
    }
}

table! {
    dispenses (tx_index) {
        tx_index -> Integer,
        dispense_index -> Nullable<Integer>,
        tx_hash -> Nullable<Text>,
        block_index -> Nullable<Integer>,
        source -> Nullable<Text>,
        destination -> Nullable<Text>,
        asset -> Nullable<Text>,
        dispense_quantity -> Nullable<Integer>,
        dispenser_tx_hash -> Nullable<Text>,
    }
}
table! {
    messages (message_index) {
        message_index -> Integer,
        block_index -> Nullable<Integer>,
        command -> Nullable<Text>,
        category -> Nullable<Text>,
        bindings -> Nullable<Text>,
        timestamp -> Nullable<Integer>,
    }
}

table! {
    sends (tx_index) {
        tx_index -> Integer,
        tx_hash -> Nullable<Text>,
        block_index -> Nullable<Integer>,
        source -> Nullable<Text>,
        destination -> Nullable<Text>,
        asset -> Nullable<Text>,
        quantity -> Nullable<Integer>,
        status -> Nullable<Text>,
        msg_index -> Nullable<Integer>,
        memo -> Nullable<Binary>,
    }
}
