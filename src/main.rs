use chrono::{DateTime, FixedOffset};
use postgres_range::{Range, range};

use clorinde::queries::module::insert_temporal_metadata;

#[tokio::main]
async fn main() {
    let pool = create_pool().await.unwrap();
    let client = pool.get().await.unwrap();

    let date_range: Range<DateTime<FixedOffset>> = {
        let offset = FixedOffset::east_opt(3600).unwrap();
        let start = DateTime::parse_from_rfc3339("2023-01-01T00:00:00+01:00")
            .unwrap()
            .with_timezone(&offset);
        let end = DateTime::parse_from_rfc3339("2023-12-31T23:59:59+01:00")
            .unwrap()
            .with_timezone(&offset);
        range!('[' start, end; ']')
    };

    let result = insert_temporal_metadata()
        .bind(&client, &date_range)
        .await
        .unwrap();

    dbg!(result);
}

use clorinde::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use clorinde::tokio_postgres::NoTls;

async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5435);
    cfg.dbname = Some(String::from("postgres"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}
