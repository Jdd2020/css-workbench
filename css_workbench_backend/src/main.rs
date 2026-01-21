use axum::{
    routing::post,
    extract::Json,
    Router,
 
};
use serde::Deserialize;

#[derive(Deserialize)]
struct CssInput {
    code: String,
}

async fn validate_css(Json(input): Json<CssInput>) -> String {
    // For MVP, just return the same CSS
    // Later, integrate syntax validation or SCSS compilation
    input.code
}

#[tokio::main]
async fn main() {
    print!("Starting CSS Playground Backend...\n");
    let app = Router::new().route("/validate_css", post(validate_css));     
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind TCP listener");
    axum::serve(listener, app)
        .await
        .unwrap();
  }
