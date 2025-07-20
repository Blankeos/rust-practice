use std::error::Error;

use dotenvy::dotenv;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};

async fn basic(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    let res = db
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT 1 + 1 as sum",
        ))
        .await?
        .unwrap();

    let sum = res.try_get::<i32>("", "sum").unwrap();
    println!("{:?} - result", sum);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db: DatabaseConnection = Database::connect(&db_url).await?;

    basic(&db).await?;

    Ok(())
}
