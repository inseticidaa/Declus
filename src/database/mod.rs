use std::env;
use actix::prelude::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PoolError};

mod schema;
mod models;
mod users;
pub mod actions;

pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;

pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn new_pool() -> Result<PgPool, PoolError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<Conn>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}
