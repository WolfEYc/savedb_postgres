use bigdecimal::ToPrimitive;
use color_eyre::{eyre::eyre, Result};
use savelib::{connect_db, PurchaseRow};
use sqlx::PgPool;
use std::collections::HashMap;
const REQ_PURCHASES_TO_FLAG: usize = 4;

const IQR_MULT: f64 = 1.5;
const BOUND_PADDING: f64 = 100_f64;

fn calc_bounds(nums: Vec<f64>) -> (f64, f64) {
    let q1 = nums[nums.len() / 4];
    let q3 = nums[3 * nums.len() / 4];

    let iqr = q3 - q1;
    let lower = q1 - IQR_MULT * iqr - BOUND_PADDING;
    let upper = q3 + IQR_MULT * iqr + BOUND_PADDING;

    (lower, upper)
}

async fn find_outliers(pool: &PgPool) -> Result<Vec<PurchaseRow>> {
    let purchases = sqlx::query_file_as!(PurchaseRow, "queries/list_all_purchases.sql")
        .fetch_all(pool)
        .await?;

    let mut recurring_purchases: HashMap<[u8; 12], Vec<PurchaseRow>> = HashMap::new();

    for purchase in purchases {
        recurring_purchases
            .entry(purchase.key())
            .or_default()
            .push(purchase);
    }

    let mut flagged_purchases: Vec<PurchaseRow> = Vec::new();

    for (_, purchases) in recurring_purchases {
        if purchases.len() < REQ_PURCHASES_TO_FLAG {
            continue;
        }
        let purchase_amts: Vec<(PurchaseRow, f64)> = purchases
            .into_iter()
            .map(|p| {
                let res = p.purchase_amount.to_f64().unwrap();
                (p, res)
            })
            .collect();

        let bounds = calc_bounds(purchase_amts.iter().map(|r| r.1).collect());

        let outlier_purchases: Vec<PurchaseRow> = purchase_amts
            .into_iter()
            .filter_map(|(row, amt)| {
                if amt < bounds.0 || amt > bounds.1 {
                    Some(row)
                } else {
                    None
                }
            })
            .collect();

        flagged_purchases.extend(outlier_purchases);
    }

    Ok(flagged_purchases)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    let pool = connect_db().await?;

    let outliers = find_outliers(&pool).await?;
    let uploadresult = 

    //list_accounts(&pool).await?;

    println!("rows_affected {}", uploadresult.rows_affected());

    Ok(())
}
