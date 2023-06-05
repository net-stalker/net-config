use std::sync::Arc;

use chrono::{DateTime, Utc, TimeZone};
use net_core::transport::sockets::{Receiver, Sender, Handler};
use postgres::{types::ToSql, Row};
use crate::{command::executor::Executor, persistence::{query, query_factory}};
use r2d2::ManageConnection;
use super::time_interval::TimeInterval;

pub struct SelectInterval<T, M>
where
    T: Sender + ?Sized,
    M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    executor: Executor<M>,
    result_receiver: Arc<T>
}
impl<T, M> query_factory::QueryFactory for SelectInterval<T, M>
where
    T: Sender + ?Sized,
    M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    type Q = SelectInterval<T, M>;
    type R = Arc<T>;
    type E = Executor<M>;
    fn create_query_handler(executor: Self::E, result_receiver: Self::R) -> Self::Q {
        SelectInterval {
            executor,
            result_receiver
        }
    }
}
struct SelectIntervalQuery<'a> {
    pub raw_query: &'a str,
    pub args: [&'a (dyn ToSql + Sync); 2]
}
impl<'a> SelectIntervalQuery<'a> {
    pub fn new(start: &'a DateTime<Utc>, end: &'a DateTime<Utc>) -> Self {
        SelectIntervalQuery { 
            raw_query: "
                SELECT
                    TIME_BUCKET('1 minute', \"frame_time\") AS bucket,
                    src_addr,
                    dst_addr 
                FROM captured_traffic
                WHERE frame_time >= $1 AND frame_time <= $2
                GROUP BY bucket, src_addr, dst_addr;
            ",
            args: [
                start,
                end
            ]
        }
    }
}
impl<'a> query::PostgresQuery<'a> for SelectIntervalQuery<'a> {
    fn get_query_params(&self) -> (&'a str, &[&'a(dyn postgres::types::ToSql + Sync)]) {
        (self.raw_query, &self.args)
    }
}
impl<T, M> SelectInterval<T, M>
where
    T: Sender + ?Sized,
    M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    pub fn select_time_interval(&self, data: TimeInterval) -> Result<Vec<Row>, postgres::Error> {
        let start = Utc.timestamp_millis_opt(data.start_interval).unwrap();
        let end = Utc.timestamp_millis_opt(data.end_interval).unwrap();
        let query = SelectIntervalQuery::new(&start, &end);
        self.executor.query(&query)
    }
}

impl<T, M> Handler for SelectInterval<T, M>
where
    T: Sender + ?Sized,
    M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("received data in SelectInterval::handle: {:?}", data);
        todo!("wait for middleware format implementation");
    }
}

#[cfg(test)]
mod tests {
    use crate::persistence::query::PostgresQuery;

    use super::*;
    #[test]
    fn select_time_interval_query_params() {
        let start = "2020-01-01 00:00:00.000 +0000".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let end = "2020-01-02 00:00:00.000 +0000".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let select_interval_query = SelectIntervalQuery::new(&start, &end);
        let (query, args) = select_interval_query.get_query_params();
        assert_eq!(query, 
            "
                SELECT
                    TIME_BUCKET('1 minute', \"frame_time\") AS bucket,
                    src_addr,
                    dst_addr 
                FROM captured_traffic
                WHERE frame_time >= $1 AND frame_time <= $2
                GROUP BY bucket, src_addr, dst_addr;
            "
        );
        assert_eq!(format!("{:?}", args), format!("{:?}", &[&start, &end]));
    }
    #[test]
    fn timestamps_from_i64_test() {
        let start_num: i64 = 1600000000000;
        let end_num: i64 = 1610000000000;
        let start = Utc.timestamp_millis_opt(start_num).unwrap();
        let end = Utc.timestamp_millis_opt(end_num).unwrap();
        assert_eq!(start, "2020-09-13 12:26:40.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap());
        assert_eq!(end, "2021-01-07 06:13:20.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap());
    }
}
