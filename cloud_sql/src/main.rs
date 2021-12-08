use actix_web::{web, HttpServer, ResponseError};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug)]
enum Error {
    InternalError(anyhow::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalError(e) => write!(f, "{}", e),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

const KEY: &'static str = "foobar";

async fn add(pool: web::Data<Pool<Postgres>>) -> Result<&'static str, Error> {
    sqlx::query!(
        r#"UPDATE my_first_table SET second_column = COALESCE(second_column, 0) + 1
            WHERE first_column = $1"#,
        KEY
    )
    .execute(pool.as_ref())
    .await
    .map_err(|e| Error::InternalError(e.into()))?;
    Ok("OK")
}

async fn get(pool: web::Data<Pool<Postgres>>) -> Result<String, Error> {
    // TODO: use transaction.
    let x = match sqlx::query!(
        r#"SELECT second_column FROM my_first_table WHERE first_column = $1"#,
        KEY,
    )
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| Error::InternalError(e.into()))?
    {
        None => 0,
        Some(x) => x.second_column.unwrap(),
    };
    Ok(format!("{}", x))
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let address = std::env::var("DATABASE_URL").unwrap();
    let pool = web::Data::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&address)
            .await?,
    );

    HttpServer::new(move || {
        actix_web::App::new()
            .route("/add", web::get().to(add))
            .route("/get", web::get().to(get))
            .app_data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
