use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use sqlx::{Database, Error, Pool, Postgres};
use sqlx::postgres::PgConnection;

#[derive(sqlx::FromRow, Debug)]
pub struct AddressPair {
    pub src_addr: String,
    pub dst_addr: String,
}

pub async fn select_address_pairs_by_date_cut<'e>(
    con: &'e Pool<Postgres>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
) -> BoxStream<'e, Result<AddressPair, Error>>
{
    sqlx::query_as::<_, AddressPair>(
        "
            SELECT src_addr, dst_addr
            FROM address_pair_aggregate
            WHERE bucket >= $1 AND bucket < $2
            GROUP BY src_addr, dst_addr
            ORDER BY src_addr, dst_addr;
        "
    )
        .bind(start_date)
        .bind(end_date)
        .fetch(con)
}
