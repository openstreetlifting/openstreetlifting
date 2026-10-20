use sqlx::postgres::{PgPool, PgPoolOptions};

pub mod error;

pub mod params;
pub mod projections;
pub mod repository;
pub mod rows;
pub mod services;

use error::Result;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    #[must_use]
    pub const fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    #[must_use]
    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Sample the live pool on scrape, rather than storing a stale background snapshot.
    pub fn record_pool_metrics(&self) {
        let size = self.pool.size();
        let idle = u32::try_from(self.pool.num_idle())
            .unwrap_or(size)
            .min(size);
        metrics::gauge!("db_client_connections", "db_system_name" => "postgresql", "state" => "used")
            .set(f64::from(size - idle));
        metrics::gauge!("db_client_connections", "db_system_name" => "postgresql", "state" => "idle")
            .set(f64::from(idle));
        metrics::gauge!("db_client_connections_max", "db_system_name" => "postgresql")
            .set(f64::from(self.pool.options().get_max_connections()));
    }

    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
