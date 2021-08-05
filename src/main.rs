mod app;
mod cli;

use app::AppInfoBuilder;
use cli::Jam;

use dotenv;
// use postgres::tls::NoTls;
use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    // let args = Jam::new();
    // let app_info = AppInfoBuilder::new(args).build();

    // println!("{:#?}", app_info);
    // println!("{:?}", dotenv::var("PG_URL").unwrap());

    let mut client = Client::connect(&dotenv::var("PG_CONFIG").unwrap(), NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    )?;

    Ok(())
}
