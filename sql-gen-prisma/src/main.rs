mod models;
use models::*;

use dotenvy::dotenv;
use sqlx::Row;
use std::error::Error;

async fn basic(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let res = sqlx::query("SELECT 1 + 1 as sum").fetch_one(pool).await?;

    let sum: i32 = res.get("sum");

    println!("Sum: {:?}", sum);

    Ok(())
}
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

async fn insert(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4().to_string();
    let username = String::from("carlo");
    let password_hash = "some_hash".to_string();
    let now = Utc::now().naive_utc();

    sqlx::query!(
        r#"
        INSERT INTO "User" (id, username, "passwordHash", "createdTimestamp", "updatedTimestamp")
        VALUES ($1, $2, $3, $4, $5)
        "#,
        id,
        username,
        password_hash,
        &now,
        &now
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn query(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let res = sqlx::query_as!(User,
        r#"SELECT id, username, "passwordHash" as password_hash, "createdTimestamp" as created_timestamp, "updatedTimestamp" as updated_timestamp FROM "User" LIMIT 1"#
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = res {
        println!("Queried user: {:?}", row);
    } else {
        println!("No user found.");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    basic(&pool).await?;
    insert(&pool).await?;
    query(&pool).await?;

    Ok(())
}
