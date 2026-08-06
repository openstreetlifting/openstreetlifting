use crate::Result;
use sqlx::PgPool;

pub struct ImportContext {
    pub pool: PgPool,
}

/// Extension point for a new competition data source.
///
/// Each source (LiftControl, spreadsheet, PDF) brings its own `Spec` type
/// describing what it needs to run.
///
/// ```ignore
/// #[async_trait::async_trait]
/// impl CompetitionImporter for LiftControlImporter {
///     type Spec = LiftControlSpec;
///
///     async fn import(&self, spec: &Self::Spec, context: &ImportContext) -> Result<()> {
///         // Implementation
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait CompetitionImporter: Send + Sync {
    type Spec: Send + Sync;

    async fn import(&self, spec: &Self::Spec, context: &ImportContext) -> Result<()>;

    fn name(&self) -> &'static str;
}
