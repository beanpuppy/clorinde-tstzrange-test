// This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub fn insert_temporal_metadata() -> InsertTemporalMetadataStmt {
    InsertTemporalMetadataStmt(crate::client::async_::Stmt::new(
        "INSERT INTO temporal_metadata ( transaction_time ) VALUES ( $1 )",
    ))
}
pub struct InsertTemporalMetadataStmt(crate::client::async_::Stmt);
impl InsertTemporalMetadataStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        transaction_time: &'a postgres_range::Range<chrono::DateTime<chrono::FixedOffset>>,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[transaction_time]).await
    }
}
