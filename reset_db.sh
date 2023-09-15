#!/bin/sh
cargo run --bin wipedb
sqlx migrate run
cargo run --bin account < data/account_info.csv
cargo run --bin purchase < data/transactions.csv
cargo run --bin outliers