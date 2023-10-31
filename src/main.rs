use axum::{routing::get, Router, ServiceExt};
mod handlers;
mod db_models;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    load_dotenv();
    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::health))
        .with_state(pool);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}",port);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
#[cfg(debug_assertions)]
fn load_dotenv(){
    dotenv().ok();
}


