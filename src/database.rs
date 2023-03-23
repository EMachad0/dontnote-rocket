use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder().test_on_check_out(true).build(manager)?;
        Ok(Self { pool })
    }

    pub fn get(&self) -> anyhow::Result<DbConn> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}
