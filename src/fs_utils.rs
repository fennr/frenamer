use anyhow::{Context, Result};
use std::{fs, path::Path};
use walkdir::WalkDir;
use crate::{rename, logger::Logger};

pub fn process_path(path: &Path, logger: Logger) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    if path.is_dir() {
        process_directory(path, logger)
    } else {
        process_file(path, logger)
    }
}

fn process_directory(dir: &Path, logger: Logger) -> Result<()> {
    logger.log_directory(dir.to_string_lossy().as_ref());
    
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        process_file(entry.path(), logger)?;
    }
    Ok(())
}

fn process_file(path: &Path, logger: Logger) -> Result<()> {
    let filename = path
        .file_name()
        .and_then(|f| f.to_str())
        .context("Invalid filename")?;

    let new_name = rename::generate_new_filename(filename)
        .with_context(|| format!("Failed to generate new name for {}", filename))?;

    if new_name != filename {
        let new_path = path.with_file_name(&new_name);
        fs::rename(path, &new_path)
            .with_context(|| format!("Failed to rename {} to {}", path.display(), new_path.display()))?;
        logger.log_rename(filename, &new_name);
    } else {
        logger.log_skip(filename);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_process_file() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("35.test-file.txt");
        File::create(&file_path)?;

        process_file(&file_path)?;

        assert!(dir.path().join("test_file_35.txt").exists());
        Ok(())
    }

    #[test]
    fn test_invalid_path() {
        let result = process_path(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }
}
