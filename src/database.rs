use {
    crate::enums::errors::{
        Error,
        Result,
    },
    diesel_async::{
        pooled_connection::{
            bb8::{
                Pool,
                PooledConnection,
            },
            AsyncDieselConnectionManager,
        },
        AsyncPgConnection,
    },
    diesel_async_migrations::{
        embed_migrations,
        EmbeddedMigrations,
    },
    futures::executor::block_on,
    once_cell::sync::Lazy,
    std::fmt::Debug,
};

pub static MIGRATIONS: Lazy<EmbeddedMigrations> = Lazy::new(|| embed_migrations!("./migrations"));

#[derive(Clone)]
pub struct Database {
    pool: Pool<AsyncPgConnection>,
    _url: String,
    _in_use: bool,
}

impl Default for Database {
    fn default() -> Self {
        block_on(Database::try_new(
            "postgresql://chatapp:123@localhost:15432/chatapp".into(),
        ))
        .unwrap()
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("url", &self._url)
            .field("_in_use", &self._in_use)
            .finish()
    }
}

impl Database {
    pub async fn try_new(url: String) -> Result<Self> {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.clone());
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .await
            .map_err(|_| Error::DatabaseConnectionFailed)?;

        let mut _conn = pool
            .get_owned()
            .await
            .map_err(Error::PoolConnectionFailed)?;
        MIGRATIONS
            .run_pending_migrations(&mut _conn)
            .await
            .map_err(|_| Error::DatabaseMigrationFailed)?;

        Ok(Database {
            pool,
            _url: url,
            _in_use: true,
        })
    }

    pub async fn get_connection(&self) -> PooledConnection<AsyncPgConnection> {
        self.pool.get().await.unwrap()
    }
}
