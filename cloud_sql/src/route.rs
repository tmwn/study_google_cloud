use actix_web::{web, ResponseError};
use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub(crate) enum Error {
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

const KEY: &str = "foobar";

pub(crate) async fn add(pool: web::Data<Pool<Postgres>>) -> Result<&'static str, Error> {
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

pub(crate) async fn get(pool: web::Data<Pool<Postgres>>) -> Result<String, Error> {
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
