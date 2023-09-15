use chrono::NaiveDate;
use color_eyre::Result;
use csv::Reader;
use savelib::*;
use serde::{Deserialize, Deserializer};
use soa_derive::StructOfArray;
use sqlx::{postgres::PgQueryResult, PgPool};
use std::{io::Stdin, usize};

const DOB_FORMAT: &str = "%m/%d/%Y";

fn deserialize_dob<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_date(deserializer, DOB_FORMAT)
}

fn deserialize_unit<'de, D>(deserializer: D) -> Result<Option<i16>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;

    if str.is_empty() {
        return Ok(None);
    };

    let parsed = str
        .replace('#', "")
        .parse()
        .map_err(serde::de::Error::custom)?;

    Ok(Some(parsed))
}

#[derive(Debug, Deserialize, StructOfArray)]
#[soa_derive[Debug]]
pub struct CSVAccount {
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
    pub ssn: String,
    pub email_address: String,
    pub mobile_number: String,
    pub account_number: i64,
}

pub async fn upload(accounts: CSVAccountVec, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file_unchecked!(
        "queries/upload_accounts.sql",
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
    )
    .execute(pool)
    .await
}

pub fn parse(mut reader: Reader<Stdin>) -> Result<CSVAccountVec> {
    reader
        .deserialize()
        .map(|r| {
            let account: CSVAccount = r?;
            Ok(account)
        })
        .collect()
}

pub async fn list_accounts(pool: &PgPool) -> Result<()> {
    let accounts = sqlx::query_file!("queries/list_all_accounts.sql")
        .fetch_all(pool)
        .await?;

    println!("{:?}", accounts);

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    let pool = connect_db().await?;
    let reader = build_reader();

    let accounts = parse(reader)?;
    let uploadresult = upload(accounts, &pool).await?;

    //list_accounts(&pool).await?;

    println!("rows_affected {}", uploadresult.rows_affected());

    Ok(())
}
