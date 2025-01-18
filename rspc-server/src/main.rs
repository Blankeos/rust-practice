use axum::{routing::get, Json};
use diesel::{
    r2d2::{self, ConnectionManager, Pool},
    QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use dotenvy::dotenv;
use rspc::{Config, Router};
use rspc_server::{
    models::{NewUser, User},
    schema::user,
};
use specta::Type;
use std::{env, path::PathBuf, sync::Arc};

// Type alias for the connection pool
type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// Your existing types
#[derive(Type, serde::Serialize)]
pub struct UserResponse {
    pub name: String,
    pub age: u8,
    pub alive: bool,
}

#[derive(Type, serde::Serialize)]
pub struct CarloResponse {
    user: UserResponse,
    greeting: String,
}

#[derive(Type, serde::Serialize, serde::Deserialize)]
pub struct RegisterInput {
    username: String,
    email: Option<String>,
    password: String,
}

#[derive(Type, serde::Serialize, serde::Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

// Application state that holds the connection pool
#[derive(Clone)]
pub struct AppState {
    pool: Arc<DbPool>,
}

fn router(state: AppState) -> Arc<Router<AppState>> {
    Router::new()
        .config(
            Config::new()
                // Doing this will automatically export the bindings when the `build` function is called.
                .export_ts_bindings(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./bindings.ts"),
                ),
        )
        // Add the procedures here.
        .query("version", |t| {
            t(|ctx: AppState, input: ()| env!("CARGO_PKG_VERSION"))
        })
        .query("carlo", |t| {
            t(|_ctx, _: ()| {
                let user = UserResponse {
                    name: "Carlo".to_string(),
                    age: 90,
                    alive: true,
                };

                CarloResponse {
                    greeting: format!("Hello there, {}!!!!", user.name),
                    user,
                }
            })
        })
        .mutation("register", |t| {
            t(|ctx, input: RegisterInput| {
                let mut conn = ctx.pool.get().expect("Failed to get db connection.");

                let hashed_password = argon2_kdf::Hasher::default()
                    .hash(input.password.as_bytes())
                    .unwrap();

                println!(
                    "[login] mutation, email:{:#?}, username:{}, password:{}, hashed_password: {}",
                    input.email,
                    input.username,
                    input.password,
                    hashed_password.to_string()
                );

                let new_user = NewUser {
                    id: uuidv7::create(),
                    username: input.username,
                    hashed_password: hashed_password.to_string(),
                    email: input.email,
                };

                let result = diesel::insert_into(user::table)
                    .values(new_user)
                    .returning(User::as_returning())
                    .get_result(&mut conn)
                    .expect("Error saving new user") as User;

                println!("this is the result: {}", result.username);

                UserResponse {
                    age: 23,
                    alive: true,
                    name: result.username,
                }
            })
        })
        .mutation("login", |t| {
            t(|ctx, input: LoginInput| {
                let mut conn = ctx.pool.get().expect("Failed to get db connection.");

                let hashed_password = argon2_kdf::Hasher::default()
                    .hash(input.password.as_bytes())
                    .unwrap();

                let result = user::table
                    .filter(user::dsl::username.eq("carlo"))
                    .select(User::as_select())
                    .get_result(&mut conn)?;
            })
        })
        .build()
        .arced()
}

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    println!("Connected to database: {}", database_url);

    pool
}

#[tokio::main]
async fn main() {
    // Create the connection pool
    let pool = establish_connection_pool();

    // Create application state
    let state = AppState {
        pool: Arc::new(pool),
    };

    // Create router with state
    let router = router(state.clone());

    // Build the application
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
            get(|| async { std::fs::read_to_string("./bindings.ts").unwrap() }),
        )
        .nest("/rspc", rspc_axum::endpoint(router, move || state.clone()));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4001").await.unwrap();
    println!("Server running on http://0.0.0.0:4001");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rspc_router() {
        // Add tests here
    }
}
