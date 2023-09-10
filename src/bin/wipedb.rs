use savelib::*;
use color_eyre::Result;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    let pool = connect_db().await?;

    sqlx::query("drop owned by qbobfvxm").execute(&pool).await?;
    Ok(())
}