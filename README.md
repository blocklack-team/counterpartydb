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
#TODO: Add more models and Schemas
#TODO: Find away to build sql query without repeat code.