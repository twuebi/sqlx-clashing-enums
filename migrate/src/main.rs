use std::path::PathBuf;

use sqlx::postgres::PgConnectOptions;
#[tokio::main]
async fn main() {
    create_schema_a().await;

    create_schema_b().await;
}
const PG_OPTIONS: &str = "PGOPTIONS";

async fn create_schema_a() {
    let value = format!("--search_path=A,public");
    std::env::set_var(PG_OPTIONS, value);
    let opts = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("password")
        .database("postgres");

    let conn = sqlx::PgPool::connect_with(opts).await.unwrap();

    sqlx::query("drop schema if exists A cascade;")
        .execute(&conn)
        .await
        .unwrap();

    sqlx::query(&"create schema if not exists A;")
        .execute(&conn)
        .await
        .unwrap();
    eprintln!("Created schema A");

    sqlx::migrate::Migrator::new(PathBuf::try_from("a/migrations").unwrap())
        .await
        .unwrap()
        .set_ignore_missing(true)
        .run(&conn)
        .await
        .unwrap();

    eprintln!("migrated A");
}

async fn create_schema_b() {
    let value = format!("--search_path=B");
    std::env::set_var(PG_OPTIONS, value);
    let opts = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("password")
        .database("postgres");

    let conn = sqlx::PgPool::connect_with(opts).await.unwrap();

    sqlx::query("drop schema if exists B cascade;")
        .execute(&conn)
        .await
        .unwrap();
    sqlx::query(&"create schema if not exists B;")
        .execute(&conn)
        .await
        .unwrap();
    eprintln!("Created schema B");
    sqlx::migrate::Migrator::new(PathBuf::try_from("b/migrations").unwrap())
        .await
        .unwrap()
        .set_ignore_missing(true)
        .run(&conn)
        .await
        .unwrap();
    eprintln!("migrated b");
}
