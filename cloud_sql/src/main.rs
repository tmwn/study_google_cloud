use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let address = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&address)
        .await?;

    sqlx::query!(
        r#"INSERT INTO my_first_table (first_column, second_column)
            VALUES ($1, $2)"#,
        "hoge",
        1234
    )
    .execute(&pool)
    .await?;

    let x = sqlx::query!(r#"SELECT first_column FROM my_first_table"#)
        .fetch_one(&pool)
        .await?
        .first_column
        .unwrap();
    println!("{}", x);

    Ok(())
}
