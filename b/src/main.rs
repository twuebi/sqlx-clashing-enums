use sqlx::postgres::PgConnectOptions;
#[tokio::main]
async fn main() {
    try_insert_b().await;
}
const PG_OPTIONS: &str = "PGOPTIONS";

#[derive(sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE", type_name = "SOME_ENUM")]
pub enum SomeEnum {
    A,
    B,
}

async fn try_insert_b() {
    let value = format!("--search_path=B,public");
    std::env::set_var(PG_OPTIONS, value);
    let opts = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("password")
        .database("postgres");

    let conn = sqlx::PgPool::connect_with(opts).await.unwrap();
    sqlx::query!(
        r#"INSERT INTO other_thing(that_enum) VALUES ($1);"#,
        SomeEnum::A as _
    )
    .execute(&conn)
    .await
    .unwrap();
}
