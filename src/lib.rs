use chrono::{NaiveDate, NaiveDateTime};
use color_eyre::Result;
use csv::{Reader, ReaderBuilder, Trim};
use serde::{Deserialize, Deserializer};
use sqlx::{types::BigDecimal, FromRow, PgPool, Pool, Postgres};
use std::{
    env,
    io::{self, Stdin},
};

pub const BIND_LIMIT: usize = u16::MAX as usize;

pub fn build_reader() -> Reader<Stdin> {
    ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin())
}

pub async fn connect_db() -> Result<Pool<Postgres>> {
    let connection_str = &env::var("DATABASE_URL")?;
    Ok(PgPool::connect(connection_str).await?)
}

pub fn deserialize_date<'de, D>(
    deserializer: D,
    format: &'static str,
) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, format).map_err(serde::de::Error::custom)
}

pub fn deserialize_datetime<'de, D>(
    deserializer: D,
    format: &'static str,
) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, format).map_err(serde::de::Error::custom)
}

#[derive(Debug, FromRow, Clone)]
pub struct PurchaseRow {
    pub account_number: i64,
    pub purchase_datetime: NaiveDateTime,
    pub purchase_amount: BigDecimal,
    pub post_date: NaiveDate,
    pub purchase_number: i32,
    pub merchant_number: String,
    pub merchant_name: String,
    pub merchant_state: String,
    pub merchant_category_code: i16,
}

impl PurchaseRow {
    pub fn key(&self) -> [u8; 12] {
        let mut pkey = [0; 12];
        pkey[..8].copy_from_slice(&self.account_number.to_ne_bytes());
        pkey[8..].copy_from_slice(&self.purchase_number.to_ne_bytes());
        pkey
    }
}
