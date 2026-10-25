use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub cache_enabled: bool,
    pub metrics_host: String,
    pub metrics_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: std::env::var("HOST").context("Cannot load HOST env variable")?,
            port: std::env::var("PORT")
                .context("PORT must be a number")?
                .parse()?,
            database_url: std::env::var("DATABASE_URL")
                .context("Cannot load DATABASE_URL env variable")?,
            cache_enabled: parse_cache_enabled(std::env::var("CACHE_ENABLED").ok().as_deref())?,
            metrics_host: std::env::var("METRICS_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            metrics_port: std::env::var("METRICS_PORT")
                .unwrap_or_else(|_| "9090".into())
                .parse()
                .context("METRICS_PORT must be a port number")?,
        })
    }
}

fn parse_cache_enabled(value: Option<&str>) -> Result<bool> {
    value
        .unwrap_or("false")
        .parse()
        .context("CACHE_ENABLED must be true or false")
}

#[cfg(test)]
mod tests {
    use super::parse_cache_enabled;

    #[test]
    fn validates_cache_settings() {
        assert!(!parse_cache_enabled(None).unwrap());
        assert!(!parse_cache_enabled(Some("false")).unwrap());
        assert!(parse_cache_enabled(Some("true")).unwrap());
        assert!(parse_cache_enabled(Some("yes")).is_err());
        assert!(parse_cache_enabled(Some("")).is_err());
    }
}
