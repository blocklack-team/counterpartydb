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

- `get_balances`: A function to query balance-related data from the database based on provided filters, limit, and offset.

- `get_blocks`: Similar to `get_balances`, but for querying block-related data.

## Endpoint

- `query_data`: An async Actix-web endpoint that processes incoming `QueryData` JSON requests. It determines the type of query based on the `method` field in `QueryData`, applies filters, and returns the corresponding data.

## Example Usage

Clients can send a JSON request specifying the query type, filters, limit, and offset:
localhost:8080/api

```json
{
    "method": "get_blocks",
    "filters": [
        {
            "field": "block_index",
            "value": {"Integer64": 100},
            "operator": ">"
        }
    ],
    "limit": 10,
    "offset": 0
}```

```json {
    "method": "get_balances",
    "filters": [
        {
            "field": "address",
            "value": {
                "String": "1Pcpxw6wJwXABhjCspe3CNf3gqSeh6eien"
            },
            "operator": "="
        },
                {
            "field": "asset",
            "value": {
                "String": "XCP"
            },
            "operator": "="
        }
    ],
    "limit": 10,
    "offset": 0
}
```
