use chrono::NaiveDate;
use savelib::*;
use csv::Reader;
use serde::{Deserialize, Deserializer};
use soa_derive::StructOfArray;
use sqlx::{PgPool, postgres::PgQueryResult};
use std::{io::Stdin, usize};
use color_eyre::Result;

const ACCOUNT_ARGS: usize = 12;
const ACCOUNT_CHUNK: usize = BIND_LIMIT / ACCOUNT_ARGS;

const DOB_FORMAT: &'static str = "%m/%d/%Y";

fn deserialize_dob<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_date(deserializer, DOB_FORMAT)
}

fn deserialize_ssn<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .replace("-", "")
        .parse()
        .map_err(serde::de::Error::custom)
}

fn deserialize_unit<'de, D>(deserializer: D) -> Result<Option<i16>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;

    if str.is_empty() {
        return Ok(None);
    };

    let parsed: i16 = str
        .replace("#", "")
        .parse()
        .map_err(serde::de::Error::custom)?;

    Ok(Some(parsed))
}

#[derive(Debug, Deserialize, StructOfArray)]
pub struct Account {
    pub last_name: String,
    pub first_name: String,
    pub street_address: String,
    #[serde(deserialize_with = "deserialize_unit")]
    pub unit: Option<i16>,
    pub city: String,
    pub state: String,
    pub zip: i32,
    #[serde(deserialize_with = "deserialize_dob")]
    pub dob: NaiveDate,
    #[serde(deserialize_with = "deserialize_ssn")]
    pub ssn: i32,
    pub email_address: String,
    pub mobile_number: i64,
    pub account_number: i64,
}

pub async fn upload(accounts: AccountVec, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!("queries/upload_accounts.sql", 
        accounts.account_number.as_slice(),
        accounts.mobile_number.as_slice(),
        accounts.email_address.as_slice(),
        accounts.ssn.as_slice(),
        accounts.dob.as_slice(),
        accounts.zip.as_slice(),
        accounts.state.as_slice(),
        accounts.city.as_slice(),
        accounts.unit.as_slice(),
        accounts.street_address.as_slice(),
        accounts.first_name.as_slice(),
        accounts.last_name.as_slice()
    ).execute(pool)
    .await
}

pub fn parse(mut reader: Reader<Stdin>) -> Result<AccountVec> {
    reader
        .deserialize()
        .map(|r| {
            let account: Account = r?;
            Ok(account)
        })
        .collect()
}

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    let pool = connect_db().await?;
    let reader = build_reader();
    
    let accounts = parse(reader)?;
    let uploadresult = upload(accounts, &pool).await?;
    
    println!("rows_affected {}", uploadresult.rows_affected());

    Ok(())
}
