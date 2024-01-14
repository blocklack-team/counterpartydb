// @generated automatically by Diesel CLI.

diesel::table! {
    assets (asset_id) {

        asset_id -> Text,
        asset_name -> Nullable<Text>,
        block_index -> Nullable<Integer>,
        asset_longname -> Nullable<Text>,
    }
}

diesel::table! {
    balances (address) {
        address -> Text,
        asset -> Nullable<Text>,
        quantity -> Nullable<Integer>,
    }
}

diesel::table! {
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
