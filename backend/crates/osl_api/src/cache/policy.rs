use std::time::Duration;

/// Keep dataset freshness decisions here, separate from deployment configuration.
#[derive(Clone, Copy)]
pub struct CachePolicy {
    pub name: &'static str,
    pub ttl: Duration,
}

pub const RIS_DISTRIBUTION: CachePolicy = CachePolicy {
    name: "ris_distribution",
    ttl: Duration::from_secs(60 * 60),
};
