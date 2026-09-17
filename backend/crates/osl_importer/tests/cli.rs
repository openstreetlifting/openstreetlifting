mod common;

use std::path::PathBuf;
use std::process::{Command, Output};

use osl_importer::canonical::{models::CanonicalFormat, store};
use sqlx::{ConnectOptions, PgPool};
use uuid::Uuid;

struct Workspace(PathBuf);

impl Workspace {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("osl-cli-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn write(&self, canonical: &CanonicalFormat) -> PathBuf {
        let directory = self
            .0
            .join("test-federation/2026")
            .join(&canonical.competition.slug);
        std::fs::create_dir_all(&directory).unwrap();
        store::write(&directory, canonical).unwrap();
        directory
    }

    fn run(&self, args: &[&str], pool: Option<&PgPool>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_import"));
        command
            .current_dir(&self.0)
            .env_remove("DATABASE_URL")
            .env_remove("OSL_PRIVACY_KEY");
        if let Some(pool) = pool {
            command.env(
                "DATABASE_URL",
                pool.connect_options().to_url_lossy().as_str(),
            );
        }
        command.args(args).output().unwrap()
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn fixture(slug: &str, name: &str) -> CanonicalFormat {
    common::competition(
        slug,
        vec![common::men_80(vec![common::lifting(
            common::athlete("Fixture", name),
            ["40", "60", "80", "120"],
        )])],
    )
}

fn success(output: Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

async fn snapshot(pool: &PgPool) -> Vec<String> {
    let mut rows = Vec::new();
    for query in [
        "SELECT COALESCE(jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text), '[]'::jsonb)::text FROM competitions t",
        "SELECT COALESCE(jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text), '[]'::jsonb)::text FROM athletes t",
        "SELECT COALESCE(jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text), '[]'::jsonb)::text FROM competition_participants t",
        "SELECT COALESCE(jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text), '[]'::jsonb)::text FROM lifts t",
        "SELECT COALESCE(jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text), '[]'::jsonb)::text FROM attempts t",
        "SELECT COALESCE(jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text), '[]'::jsonb)::text FROM athlete_socials t",
    ] {
        rows.push(sqlx::query_scalar(query).fetch_one(pool).await.unwrap());
    }
    rows
}

#[test]
fn dry_run_validates_files_and_legacy_flags_remain_usable() {
    let workspace = Workspace::new();
    let directory = workspace.write(&fixture("meet", "Alpha"));
    let path = directory.to_str().unwrap();
    for args in [
        vec!["competitions", path, "--dry-run", "--prune"],
        vec!["canonical", path, "--validate-only"],
        vec![
            "bulk-import",
            "--directory",
            path,
            "--validate-only",
            "--prune",
            "--yes",
        ],
    ] {
        success(workspace.run(&args, None));
    }
    let overlapping = workspace.run(&["competitions", ".", path, "--dry-run"], None);
    success(overlapping);
    for args in [
        vec!["competitions", "missing", "--dry-run"],
        vec!["competitions", path, "missing", "--dry-run"],
        vec!["competitions", path, "--skip-privacy-check"],
    ] {
        assert!(!workspace.run(&args, None).status.success());
    }
}

#[test]
fn formatting_preview_and_check_leave_files_unchanged() {
    let workspace = Workspace::new();
    let directory = workspace.write(&fixture("meet", "Alpha"));
    let path = directory.join("competition.toml");
    let contents = format!("\n{}", std::fs::read_to_string(&path).unwrap());
    std::fs::write(&path, &contents).unwrap();
    success(workspace.run(&["fmt", ".", "--dry-run"], None));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), contents);
    assert!(
        !workspace
            .run(&["fmt", ".", "--check"], None)
            .status
            .success()
    );
    assert_eq!(std::fs::read_to_string(&path).unwrap(), contents);
    success(workspace.run(&["fmt", "."], None));
    success(workspace.run(&["fmt", ".", "--check"], None));
}

#[test]
fn help_hides_credentials_and_redaction_validates_identity_options() {
    let workspace = Workspace::new();
    let output = Command::new(env!("CARGO_BIN_EXE_import"))
        .current_dir(&workspace.0)
        .env(
            "DATABASE_URL",
            "postgres://synthetic-password@localhost/test",
        )
        .args(["redact", "--help"])
        .output()
        .unwrap();
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(help.contains("--database-url"));
    assert!(!help.contains("synthetic-password"));
    workspace.write(&fixture("meet", "Alpha"));
    for args in [
        vec!["redact", "--name", " ", "--dry-run"],
        vec![
            "redact",
            "--name",
            "Fixture Alpha",
            "--sex",
            "male",
            "--dry-run",
        ],
        vec![
            "redact",
            "--name",
            "Fixture Alpha",
            "--country",
            "France",
            "--dry-run",
        ],
        vec![
            "redact",
            "--name",
            "Fixture Alpha",
            "--disambiguation",
            "0",
            "--dry-run",
        ],
    ] {
        assert_eq!(workspace.run(&args, None).status.code(), Some(2));
    }
    let output = Command::new(env!("CARGO_BIN_EXE_import"))
        .current_dir(&workspace.0)
        .env("OSL_PRIVACY_KEY", "test-key")
        .args([
            "redact",
            "--name",
            "Fixture Alpha",
            "--sex",
            "m",
            "--country",
            "fr",
            "--directory",
            ".",
            "--dry-run",
        ])
        .output()
        .unwrap();
    success(output);
    assert!(!workspace.0.join("data/athletes/privacy.csv").exists());
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn dry_run_and_invalid_inputs_cannot_import_or_prune(pool: PgPool) {
    let workspace = Workspace::new();
    common::import(&pool, fixture("kept", "Alpha")).await;
    common::import(&pool, fixture("removed", "Bravo")).await;
    workspace.write(&fixture("kept", "Charlie"));
    let before = snapshot(&pool).await;
    success(workspace.run(&["competitions", ".", "--dry-run", "--prune"], Some(&pool)));
    success(workspace.run(
        &[
            "bulk-import",
            "--directory",
            ".",
            "--dry-run",
            "--prune",
            "--yes",
        ],
        Some(&pool),
    ));
    assert_eq!(snapshot(&pool).await, before);
    assert!(
        !workspace
            .run(&["competitions", ".", "missing", "--prune"], Some(&pool))
            .status
            .success()
    );
    assert_eq!(snapshot(&pool).await, before);
    let bad = workspace.write(&fixture("invalid", "Invalid"));
    std::fs::write(bad.join("entries.csv"), "not valid csv").unwrap();
    assert!(
        !workspace
            .run(&["competitions", ".", "--prune"], Some(&pool))
            .status
            .success()
    );
    assert_eq!(snapshot(&pool).await, before);
    std::fs::remove_dir_all(bad).unwrap();
    success(workspace.run(&["competitions", ".", "--prune"], Some(&pool)));
    let names: Vec<String> =
        sqlx::query_scalar("SELECT last_name FROM athletes ORDER BY last_name")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(names, vec!["Charlie"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn existing_kubernetes_arguments_still_import_and_prune(pool: PgPool) {
    let workspace = Workspace::new();
    common::import(&pool, fixture("removed", "Bravo")).await;
    workspace.write(&fixture("kept", "Alpha"));
    success(workspace.run(
        &["bulk-import", "--directory", ".", "--prune", "--yes"],
        Some(&pool),
    ));
    let slugs: Vec<String> = sqlx::query_scalar("SELECT slug FROM competitions")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(slugs, vec!["kept"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn instagram_and_recompute_previews_leave_stored_data_unchanged(pool: PgPool) {
    let workspace = Workspace::new();
    common::import(&pool, fixture("meet", "Alpha")).await;
    std::fs::write(
        workspace.0.join("instagram.csv"),
        "Name,Instagram\nFixture Alpha,fixture_alpha\n",
    )
    .unwrap();
    sqlx::query("UPDATE competition_participants SET ris_score = 1")
        .execute(&pool)
        .await
        .unwrap();
    let before = snapshot(&pool).await;
    success(workspace.run(&["instagram", "instagram.csv", "--dry-run"], Some(&pool)));
    success(workspace.run(&["recompute-ris", "--dry-run"], Some(&pool)));
    assert_eq!(snapshot(&pool).await, before);
    success(workspace.run(&["instagram", "instagram.csv"], Some(&pool)));
    success(workspace.run(&["recompute-ris"], Some(&pool)));
    assert_ne!(snapshot(&pool).await, before);
    let handles: Vec<String> = sqlx::query_scalar("SELECT handle FROM athlete_socials")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(handles, vec!["fixture_alpha"]);
}
