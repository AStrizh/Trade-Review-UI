mod models;
mod routes;

use std::net::SocketAddr;
use http::{header, HeaderValue, Method};

use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trade_review_backend=debug,tower_http=debug,axum=debug".into()),
        )
        .init();

	let cors = CorsLayer::new()
		.allow_origin(HeaderValue::from_static("http://localhost:5173"))
		.allow_methods([Method::GET])
		.allow_headers([header::CONTENT_TYPE])
		.allow_credentials(false)
		.expose_headers(Any);


    let app = routes::app_router().layer(TraceLayer::new_for_http()).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind backend listener");

    tracing::info!("backend listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("backend server should run");
}
