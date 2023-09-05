use chrono::{NaiveDateTime, NaiveDate};
use savelib::*;
use csv::Reader;
use serde::{Deserialize, Deserializer};
use soa_derive::StructOfArray;
use sqlx::{PgPool, postgres::PgQueryResult, types::BigDecimal};
use std::{io::Stdin, str::FromStr};
use color_eyre::Result;

const PURCHASE_DATETIME_FORMAT: &'static str = "%m%d%Y %H:%M:%S";
const POST_DATE_FORMAT: &'static str = "%m%d%Y";

fn deserialize_purchase_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_datetime(deserializer, PURCHASE_DATETIME_FORMAT)
}

fn deserialize_post_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_date(deserializer, POST_DATE_FORMAT)
}

fn deserialize_transaction_amount<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;

    match s.pop() {
        None => Err(serde::de::Error::custom("Empty transaction amt")),
        Some(op) => {

            if op == '-' {
                s = format!("{}{}", op, s);
            }

            let dec = BigDecimal::from_str(&s)
                .map_err(serde::de::Error::custom)?
                .with_prec(10)
                .with_scale(2);
            
            //println!("{}", dec);

            Ok(dec)
        }
    }
}

fn is_word(s: &String) -> bool {
    s.chars().all(|c| c.is_alphabetic())
}

fn fix_legagy_merchant_name(purchase: &mut Purchase) {
    if is_word(&purchase.merchant_number) {
        purchase.merchant_description.merchant_name = purchase.merchant_number.clone();
    };
}

impl<'de> Deserialize<'de> for MerchantDescription {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let split = s
            .rsplit_once(" ")
            .ok_or(serde::de::Error::custom("Merchant name parse error"))?;
    
        let name: String = split.0.split_whitespace().collect::<Vec<&str>>().join(" ");
    
        let state: String = split
            .1
            .get(..2)
            .ok_or(serde::de::Error::custom("Merchant state not found"))?
            .to_string();
    
        //println!("name: {name} state: {state}");
        Ok(MerchantDescription {
            merchant_name: name,
            merchant_state: state.to_string(),
        })
    }
}

#[derive(Debug, StructOfArray)]
pub struct MerchantDescription {
    pub merchant_name: String,
    pub merchant_state: String,
}

#[derive(Debug, Deserialize, StructOfArray)]
pub struct Purchase {
    pub account_number: i64,
    #[serde(deserialize_with = "deserialize_purchase_datetime")]
    pub transaction_datetime: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_transaction_amount")]
    pub transaction_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_post_date")]
    pub post_date: NaiveDate,
    pub transaction_number: i32,
    pub merchant_number: String,
    #[nested_soa]
    pub merchant_description: MerchantDescription,
    pub merchant_category_code: i16,
}

pub async fn upload(purchases: &PurchaseVec, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!("queries/upload_purchases.sql", 
        purchases.account_number.as_slice(),
        purchases.transaction_datetime.as_slice(),
        purchases.transaction_amount.as_slice(),
        purchases.post_date.as_slice(),
        purchases.transaction_number.as_slice(),
        purchases.merchant_number.as_slice(),
        purchases.merchant_description.merchant_name.as_slice(),
        purchases.merchant_description.merchant_state.as_slice(),
        purchases.merchant_category_code.as_slice()
    ).execute(pool)
    .await
}

pub fn parse(mut reader: Reader<Stdin>) -> Result<PurchaseVec> {
    /*
    for str_purchase in reader.records().take(1) {
        println!("{:?}", str_purchase);
    }
    */
    reader
        .deserialize()
        .map(|r| {
            let mut account: Purchase = r?;
            fix_legagy_merchant_name(&mut account);
            Ok(account)
        })
        .collect()
}

pub async fn list_all_purchases(pool: &PgPool) -> Result<()> {
    let purchases = sqlx::query_file!("queries/list_all_purchases.sql")
        .fetch_all(pool)
        .await?;

    for purchase in purchases {
        println!("{:?}", purchase.purchase_amount.to_string().parse::<f64>());
    }

    Ok(())
}

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    let pool = connect_db().await?;
    let reader = build_reader();

    let purchases = parse(reader)?;
    let uploadresult = upload(&purchases, &pool).await?;

    //list_all_purchases(&pool).await?;

    println!("rows_affected {}", uploadresult.rows_affected());

    Ok(())
}
