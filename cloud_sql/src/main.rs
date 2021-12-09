use actix_web::{web, HttpServer};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use structopt::StructOpt;

mod route;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let opt = Opt::from_args();

    // let address = std::env::var("DATABASE_URL").unwrap();
    let options = {
        PgConnectOptions::new()
            .host(&opt.db_host)
            .database(&opt.db_name)
            .password(&opt.db_password)
            .username(&opt.db_user)
    };

    let pool = web::Data::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?,
    );

    HttpServer::new(move || {
        actix_web::App::new()
            .route("/add", web::get().to(route::add))
            .route("/get", web::get().to(route::get))
            .app_data(pool.clone())
    })
    .bind(format!("0.0.0.0:{}", opt.port))?
    .run()
    .await?;
    Ok(())
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, env)]
    port: u16,
    // DB options. https://docs.rs/sqlx/0.5.9/sqlx/postgres/struct.PgConnectOptions.html
    #[structopt(long, env)]
    db_host: String,
    #[structopt(long, env)]
    db_name: String,
    #[structopt(long, env)]
    db_password: String,
    #[structopt(long, env)]
    db_user: String,
}
