use color_eyre::Result;
use savelib::{connect_db, PurchasePKeyVec, PurchaseRow, RecurringPurchase};
use sqlx::{postgres::PgQueryResult, PgPool};
use std::collections::HashMap;
const REQ_PURCHASES_TO_FLAG: usize = 4;

const IQR_MULT: f64 = 1.5;
const BOUND_PADDING: f64 = 100_f64;

fn calc_bounds(nums: Vec<f64>) -> (f64, f64) {
    let q1 = nums[nums.len() / 4];
    let q3 = nums[3 * nums.len() / 4];

    let iqr = q3 - q1;
    let lower = (q1 - IQR_MULT * iqr) - BOUND_PADDING;
    let upper = (q3 + IQR_MULT * iqr) + BOUND_PADDING;

    (lower, upper)
}

async fn find_outliers(pool: &PgPool) -> Result<Vec<PurchaseRow>> {
    let purchases = sqlx::query_file_as!(PurchaseRow, "queries/list_all_purchases.sql")
        .fetch_all(pool)
        .await?;

    let mut recurring_purchases: HashMap<RecurringPurchase, Vec<PurchaseRow>> = HashMap::new();

    for purchase in purchases {
        recurring_purchases
            .entry(purchase.clone().into())
            .or_default()
            .push(purchase);
    }

    let mut flagged_purchases: Vec<PurchaseRow> = Vec::new();

    for purchases in recurring_purchases.into_values() {
        if purchases.len() < REQ_PURCHASES_TO_FLAG {
            continue;
        }

        let amts = purchases.iter().map(|r| r.purchase_amount.0).collect();
        let bounds = calc_bounds(amts);

        println!("{:?}", bounds);

        let outlier_purchases: Vec<PurchaseRow> = purchases
            .into_iter()
            .filter(|r| r.purchase_amount.0 < bounds.0 || r.purchase_amount.0 > bounds.1)
            .collect();

        println!("{:#?}", outlier_purchases);

        flagged_purchases.extend(outlier_purchases);
    }

    Ok(flagged_purchases)
}

async fn upload_outliers(
    outliers: Vec<PurchaseRow>,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    let outliers: PurchasePKeyVec = outliers.into_iter().map(|r| r.into()).collect();
    sqlx::query_file!(
        "queries/upload_outliers.sql",
        outliers.account_number.as_slice(),
        outliers.purchase_number.as_slice()
    )
    .execute(pool)
    .await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    let pool = connect_db().await?;
    let outliers = find_outliers(&pool).await?;
    let uploadresult = upload_outliers(outliers, &pool).await?;

    println!("rows_affected {}", uploadresult.rows_affected());

    Ok(())
}
