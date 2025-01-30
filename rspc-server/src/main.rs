use dotenvy::dotenv;

use axum::{routing::get, Json};
use rspc::{Config, Router};
use specta::Type;
use std::{env, path::PathBuf, sync::Arc};

// Type is for exporting types to Typescript,
// Serde is for struct -> JSON serialization more for runtime.
#[derive(Type, serde::Serialize)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub alive: bool,
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

fn router(conn: &mut SqliteConnection) -> Arc<Router<()>> {
    <Router>::new()
        .config(
            Config::new()
                // Doing this will automatically export the bindings when the `build` function is called.
                .export_ts_bindings(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./bindings.ts"),
                ),
        )
        // Add the procedures here.
        .query("version", |t| t(|ctx, input: ()| env!("CARGO_PKG_VERSION")))
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

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    // Pool Implementation (Needed for)
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // let manager = ConnectionManager::<SqliteConnection>::new(database_url.clone());
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool");

    // println!("Connected to database: {}!", database_url);

    // Non-Pool Implementation
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    println!("Connected to database: {}!", database_url);

    connection
}

#[tokio::main]
async fn main() {
    let mut conn = &mut establish_connection();

    let _router = router(&mut conn);

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
        .nest("/rspc", rspc_axum::endpoint(_router, || ()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
