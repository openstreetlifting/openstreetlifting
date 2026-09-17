use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use uuid::Uuid;

pub(crate) struct Replacement {
    temporary: PathBuf,
    destination: PathBuf,
}

impl Replacement {
    pub(crate) fn prepare(destination: &Path, contents: &[u8]) -> Result<Self> {
        let parent = destination
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        std::fs::create_dir_all(parent)?;
        let replacement = Self {
            temporary: parent.join(format!(".osl-{}.tmp", Uuid::new_v4())),
            destination: destination.to_path_buf(),
        };
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&replacement.temporary)
            .with_context(|| format!("staging {}", destination.display()))?;
        if let Ok(metadata) = std::fs::metadata(destination) {
            anyhow::ensure!(
                metadata.is_file(),
                "{} is not a file",
                destination.display()
            );
            file.set_permissions(metadata.permissions())?;
        }
        file.write_all(contents)?;
        file.sync_all()?;
        Ok(replacement)
    }

    pub(crate) fn commit(self) -> Result<()> {
        std::fs::rename(&self.temporary, &self.destination)
            .with_context(|| format!("replacing {}", self.destination.display()))?;
        let parent = self
            .destination
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        File::open(parent)?.sync_all()?;
        Ok(())
    }
}

impl Drop for Replacement {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.temporary);
    }
}
