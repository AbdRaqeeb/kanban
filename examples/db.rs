use sqlx::{Connection, PgConnection, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

async fn use_pool(pool: &Pool<Postgres>) {
    // don't need to fetch connection from pool
    // can pass pool directly to queries, e.g.
    sqlx::query_as::<_, (String, )>("SELECT version()")
        .fetch_one(pool) // passing pool directly
        .await
        .unwrap();
}

// example of connecting to PostgreSQL
async fn get_connection() -> PgConnection {
    dotenv::dotenv().unwrap();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgConnection::connect(&db_url).await.unwrap()
}

fn main() {}
