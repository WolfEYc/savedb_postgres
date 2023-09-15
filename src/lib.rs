use bigdecimal::ToPrimitive;
use chrono::{NaiveDate, NaiveDateTime};
use color_eyre::Result;
use csv::{Reader, ReaderBuilder, Trim};
use serde::{Deserialize, Deserializer};
use soa_derive::StructOfArray;
use sqlx::{types::BigDecimal, FromRow, PgPool, Pool, Postgres};
use std::{
    env,
    hash::Hash,
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

#[derive(Debug, Clone)]
pub struct PurchaseAmt(pub f64);

impl From<BigDecimal> for PurchaseAmt {
    fn from(value: BigDecimal) -> PurchaseAmt {
        PurchaseAmt(value.to_f64().unwrap())
    }
}

#[derive(Debug, StructOfArray)]
pub struct PurchasePKey {
    pub account_number: i64,
    pub purchase_number: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RecurringPurchase {
    pub account_number: i64,
    pub merchant_number: String,
}

#[derive(Debug, FromRow, Clone)]
pub struct PurchaseRow {
    pub account_number: i64,
    pub purchase_datetime: NaiveDateTime,
    #[sqlx(try_from = "BigDecimal")]
    pub purchase_amount: PurchaseAmt,
    pub post_date: NaiveDate,
    pub purchase_number: i32,
    pub merchant_number: String,
    pub merchant_name: String,
    pub merchant_state: String,
    pub merchant_category_code: i16,
}

impl From<PurchaseRow> for PurchasePKey {
    fn from(value: PurchaseRow) -> Self {
        PurchasePKey {
            account_number: value.account_number,
            purchase_number: value.purchase_number,
        }
    }
}

impl From<PurchaseRow> for RecurringPurchase {
    fn from(value: PurchaseRow) -> Self {
        RecurringPurchase {
            account_number: value.account_number,
            merchant_number: value.merchant_number,
        }
    }
}
