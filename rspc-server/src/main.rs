use rspc::{selection, Router};

use axum::routing::get;
use specta::Type;
use std::sync::Arc;

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

fn router() -> Arc<Router<()>> {
    <Router>::new()
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
        .build()
        .arced()
}

#[tokio::main]
async fn main() {
    let _router = router();

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
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
        super::router();
    }
}
