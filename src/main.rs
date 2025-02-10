use axum::Router;

mod currency;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", root);
}

async fn root() -> &'static str {
    "Hello world"
}
