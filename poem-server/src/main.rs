use std::sync::Arc;

use poem::{EndpointExt, Route, get, handler, listener::TcpListener, web::Data};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(path = "/goodbye", method = "get")]
    async fn goodbye(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("goodbye, {}!", name)),
            None => PlainText("goodbye!".to_string()),
        }
    }

    #[oai(path = "/echo", method = "post")]
    async fn echo(&self, body: String) -> PlainText<String> {
        PlainText(body)
    }
}

#[handler]
fn index() -> String {
    format!("Hello poem server!")
}

#[handler]
fn docs_handler(Data(spec): Data<&Arc<String>>) -> String {
    spec.as_ref().clone()
}

#[handler]
fn ui() -> poem::web::Html<&'static str> {
    poem::web::Html(
        r#"<!doctype html>
<html>
  <head>
    <title>Scalar API Reference</title>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1" />
  </head>

  <body>
    <div id="app"></div>

    <!-- Load the Script -->
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>

    <!-- Initialize the Scalar API Reference -->
    <script>
      Scalar.createApiReference('#app', {
        // The URL of the OpenAPI/Swagger document
        url: 'http://localhost:3000/docs/json',
        // Avoid CORS issues
        proxyUrl: 'https://proxy.scalar.com',
      })
    </script>
  </body>
</html>"#,
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    // let ui = api_service.swagger_ui();

    let spec = api_service.spec();
    let app = Route::new()
        .at("/", get(index))
        .nest("/api", api_service)
        .at("/docs/json", get(docs_handler))
        .nest("/docs", ui)
        .data(Arc::new(spec));

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
