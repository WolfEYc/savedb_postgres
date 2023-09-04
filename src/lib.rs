use color_eyre::Result;
use chrono::{NaiveDate, NaiveDateTime};
use csv::{Reader, ReaderBuilder, Trim};
use serde::{Deserialize, Deserializer};
use sqlx::{Pool, Postgres, PgPool};
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

pub fn deserialize_date<'de, D>(deserializer: D, format: &'static str) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, format)
        .map_err(serde::de::Error::custom)
}

pub fn deserialize_datetime<'de, D>(
    deserializer: D,
    format: &'static str,
) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, format)
        .map_err(serde::de::Error::custom)
}
