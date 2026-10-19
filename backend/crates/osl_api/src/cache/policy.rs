use std::time::Duration;

#[derive(Clone, Copy)]
pub struct CachePolicy {
    pub name: &'static str,
    pub ttl: Duration,
}

pub const RIS_DISTRIBUTION: CachePolicy = CachePolicy {
    name: "ris_distribution",
    ttl: Duration::from_hours(1),
};
