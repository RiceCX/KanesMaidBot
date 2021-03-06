use dotenv::dotenv;
use std::env;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{Pool, Postgres, ConnectOptions, Error};


async fn init_pool(database_url: &str) -> Pool<Postgres> {

    PgPoolOptions::new()
       .max_connections(2)
       .connect(database_url).await.expect("Could not open pool to the database.")
}

pub async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    init_pool(&database_url).await
}
