# This is an untested development version, please do not use it in production.
# counterpartydb
A Counterparty db wrapper

## Overview

This API enables dynamic querying of database tables, allowing clients to specify various filters, limits, and offsets to retrieve data. It's designed to be flexible, accommodating a wide range of query requirements.

## Key Components

- **FilterValue**: An enum that represents the types of values that can be used in filters. It includes variants like `String`, `Integer`, and `Integer64`.

- **DynamicFilter**: A struct that defines a single filter criterion. It includes a `field` to specify the column name, a `value` of type `FilterValue`, and an `operator` to define the comparison operation (like `>`, `<`, `=`, etc.).

- **QueryData**: A struct that encapsulates the entire query request. It includes a `method` to specify the type of query (like `get_balances` or `get_blocks`), an array of `DynamicFilter` for specifying filters, and `limit` and `offset` fields for pagination control.

- **QueryResult**: An enum used to represent the different possible types of query results, such as `Balances(Vec<models::Balance>)` or `Blocks(Vec<models::Block>)`.

## Query Functions

- [x] Balances
- [x] Dispensers
- [x] Burns
- [x] Issuances
- [x] Blocks
- [x] Debits
- [ ] Credits
- [ ] Cancels
- [x] Messages
- [x] Sends
- [x] Dispenses
- [x] Bets
- [ ] Dividends
- [ ] Rps
- [ ] Broadcasts
- [ ] Bet Expirations
- [ ] Order Expirations
- [ ] Bet Match expirations
- [ ] Order Match Expirations
- [ ] Bet Match Resolutions
- [ ] Mempool
- [ ] Orders
- [ ] Btcpays

## decode functions
- [x] Classic send
- [x] Enchance Send

## Address support
- [x] p2pkh
- [x] p2sh
- [x] p2sh-p2wpkh
- [x] p2wsh

## Example Usage

- configure the  `.env`
- Example
  
```env
DATABASE_URL=/db/counterparty.db
PORT=8080
```

to run server 
```
cargo run
```
production (better perfomance)
```
cargo run --release
```
## Example Curl 
  
```curl
  curl -X POST \
  http://localhost:8080/api \
  -H 'Content-Type: application/json' \
  -d "{
    "method": "get_dispensers",
    "filters": [
        {
            "field": "block_index",
            "value": 278270,
            "op": ">"
        }
    ],
    "filterop": "AND",
    "limit": 10,
    "offset": 0 }'

```

## Multiple filters

```curl
curl -X POST \
  http://localhost:8080/api \
  -H 'Content-Type: application/json' \
  -d '{
    "method": "get_balances",
    "filters": [
        {
            "field": "address",
            "value": "1AeqgtHedfA2yVXH6GiKLS2JGkfWfgyTC6",
            "op": "="
        },
        {
            "field": "asset",
            "value":  "XCP",
            "op": "="
        }
    ],
    "limit": 10,
    "offset": 0 }'
```
## join using OR & AND
```curl
curl -X POST \
  http://localhost:8080/api \
  -H 'Content-Type: application/json' \
  -d '{
    "method": "get_balances",
    "filters": [
        {
            "field": "address",
            "value": "1AeqgtHedfA2yVXH6GiKLS2JGkfWfgyTC6",
            "op": "="
        },
        {
            "field": "address",
            "value": "1LhEGAPUZnfNDbh7oFogdekUyTW8NBfW3g",
            "op": "="
        }
    ],
    "filter_op": "OR",
    "limit": 100,
    "offset": 0 }'
```
## Decoder

```curl
    curl -X POST http://127.0.0.1:8080/get_info_rawtx \
    -H "Content-Type: application/json" \
    -d '{
        "rawtx": "020000000140243285638672be73fed9434690b5eea3d927b57a2bfea34c46c62920f569f6010000008a473044022012de14728871729d50662588cb03e19ce529fa77181d436ee3f8d202ec03556f02202255eaa418eb1582fbcde2f185d334a6f2ae7fb931c997e679d7841b21f970fa01410432f573c42f761063ce6894d4cddcc346d73d435deb712dd6eae5962dcd527afa9f7da26ae9686f4bb646462567ffdd0b9d070f54f4e328bf07bedbcc65be432bffffffff020000000000000000336a3140b92ffc35b6ee32c66e9cacd427040d0233f344f61fa5ae9af787beee89f58c5bea36ae773c85bfda49c169cb917d7517a31a2c01000000001976a9145949b02d02299682a2f168235644edd169dab2f388ac00000000"
    }'
```
### Result

```json
    {
        "asset_id": 1,
        "asset_name": "XCP",
        "quantity": 460000000000,
        "recipient": "bc1query8r32jhda5x9zrlstlvhdhlaqusxet2kgfk",
        "memo": "OTC",
        "source": "1997R23AD1F8W4xU8Rved8akpectBUi25u"
    }
```

#TODO: Add more models and Schemas
#TODO: Find away to build sql query without repeat code.