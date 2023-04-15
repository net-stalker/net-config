use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use postgres::NoTls;
use std::sync::{Arc, Mutex};
use crate::db_access::query;

#[derive(Clone)]
pub struct Executor{
    pub connection_pool: Arc<Mutex<Pool<PostgresConnectionManager<NoTls>>>>
}

impl Executor{
    pub fn new(connection_pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Executor { connection_pool: Arc::new(Mutex::new(connection_pool)) }
    }
    fn get_connection(&self) -> PooledConnection<PostgresConnectionManager<NoTls>> {
        self.connection_pool.lock()
        .unwrap()
        .get()
        .unwrap()
    }
    pub fn execute<'a, Q>(&self, query: &'a Q) -> Result<u64, postgres::Error>
    where
        Q: query::PostgresQuery<'a>
    {
        let (query_string, params) = query.get_query_params();
        self.get_connection().execute(query_string, params)
    }
    pub fn query<'a, Q>(&self, query: &'a Q) -> Result<Vec<postgres::Row>, postgres::Error>
    where
        Q: query::PostgresQuery<'a>
    {
        let (query_string, params) = query.get_query_params();
        self.get_connection().query(query_string, params)
    }
}