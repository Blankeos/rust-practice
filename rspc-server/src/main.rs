use dotenvy::dotenv;

use axum::{routing::get, Json};
use rspc::Config;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use specta::Type;
use std::{env, error::Error, path::PathBuf, sync::Arc};

// Type is for exporting types to Typescript,
// Serde is for struct -> JSON serialization more for runtime.
#[derive(Type, serde::Serialize)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub alive: bool,
}

#[derive(Type, serde::Serialize)]
pub struct NewUser {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub email: Option<String>,
}

#[derive(Type, serde::Serialize)]
pub struct CarloResponse {
    user: User,
    greeting: String,
}

#[derive(Type, serde::Serialize)]
pub struct LoginInput {
    username: String,
    email: String,
    password: String,
}

fn router() -> Arc<rspc::Router<MyCtx>> {
    rspc::Router::<MyCtx>::new()
        .config(
            Config::new()
                // Doing this will automatically export the bindings when the `build` function is called.
                .export_ts_bindings(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./bindings.ts"),
                ),
        )
        // Add the procedures here.
        .query("version", |t| t(|ctx, input: ()| env!("CARGO_PKG_VERSION")))
        .query("sum", |t| {
            t(async |ctx, input: ()| {
                let res = ctx
                    .db
                    .query_one(sea_orm::Statement::from_string(
                        sea_orm::DatabaseBackend::Sqlite,
                        "SELECT 1 + 1 as sum",
                    ))
                    .await
                    .unwrap();

                let sum = res
                    .expect("Expected a result from the database query")
                    .try_get::<i32>("", "sum")
                    .unwrap();

                Ok(sum)
            })
        })
        .query("get_all_users", |t| {
            t(async |ctx, _: ()| {
                let users = entity::user::Entity::find()
                    .all(&*ctx.db)
                    .await
                    .map_err(|e| {
                        rspc::Error::new(
                            rspc::ErrorCode::InternalServerError,
                            format!("Failed to fetch users: {}", e),
                        )
                    })?;

                Ok(users)
            })
        })
        .query("carlo", |t| {
            t(|ctx, input: ()| {
                let name = "Carlo";
                let greeting = format!("Hello there, {}!!!!", name);

                let user = User {
                    name: name.to_string(),
                    age: 90,
                    alive: true,
                };

                return CarloResponse { greeting, user };
            })
        })
        .query("login", |t| {
            t(|ctx, input: ()| {
                let new_user = NewUser {
                    id: "123".to_string(),
                    username: "carlo".to_string(),
                    hashed_password: "carlo123".to_string(),
                    email: None,
                };

                "logging in."
            })
        })
        // .mutation("login", |t| t(|ctx, login_input: LoginInput| {}))
        .build()
        .arced()
}

#[derive(Clone)] // Clone is generally required
struct MyCtx {
    // UNLEARNED CONCEPT: Lifetimes. &'static for example? I guess Arc is one of those lifetimes?
    db: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db: DatabaseConnection = sea_orm::Database::connect(&db_url).await?;
    let arc_db = Arc::new(db);

    let _router = router();

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .route(
            "/up",
            get(|| async {
                Json(serde_json::json!({
                    "status": "up",
                    "message": "Server is running"
                }))
            }),
        )
        .route(
            "/bindings",
            get(|| async {
                let value = std::fs::read_to_string("./bindings.ts").unwrap();
                value
            }),
        )
        .nest(
            "/rspc",
            rspc_axum::endpoint(_router, || (MyCtx { db: arc_db })),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    // It is highly recommended to unit test your rspc router by creating it
    // This will ensure it doesn't have any issues and also export updated Typescript types.

    #[test]
    fn test_rspc_router() {
        // super::router();
    }
}
