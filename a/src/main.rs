use sqlx::postgres::PgConnectOptions;
#[tokio::main]
async fn main() {
    // let sql = format!("DROP DATABASE IF EXISTS postgres WITH (FORCE);");
    // sqlx::query(&sql).execute(&conn).await.unwrap();

    // let sql = format!("CREATE DATABASE postgres;");
    // sqlx::query(&sql).execute(&conn).await.unwrap();
    try_insert_a().await;
}
const PG_OPTIONS: &str = "PGOPTIONS";

#[derive(sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE", type_name = "SOME_ENUM")]
pub enum SomeEnum {
    A,
    B,
}

async fn try_insert_a() {
    let value = format!("--search_path=A,public");
    std::env::set_var(PG_OPTIONS, value);

    let opts = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("password")
        .database("postgres");

    let conn = sqlx::PgPool::connect_with(opts).await.unwrap();
    sqlx::query!(
        r#"INSERT INTO thing(some_value) VALUES ($1);"#,
        SomeEnum::A as _
    )
    .execute(&conn)
    .await
    .unwrap();
}
