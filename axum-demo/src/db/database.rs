use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be in the env.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres.");

    pool
}

pub async fn run_migrations() -> () {
    let pool = create_pool().await;

    sqlx::migrate!("src/db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations.");

    println!("Database migrations ran successfully.");
}

pub async fn refresh_database() -> () {
    println!("Database refreshing.");

    let pool = create_pool().await;

    sqlx::query("DROP SCHEMA public CASCADE;")
        .execute(&pool)
        .await
        .expect("Failed to drop schema.");

    sqlx::query("CREATE SCHEMA public;")
        .execute(&pool)
        .await
        .expect("Failed to create schema.");

    run_migrations().await;

    println!("Database refreshed successfully.");
}
