use crate::cli::output::OutputHandler;
use crate::core::{config::Config, error::CldevError, Result};
use std::fs;
use std::path::PathBuf;

pub fn handle_config_maintain(backup: bool, cleanup: bool, output: &OutputHandler) -> Result<()> {
    let config_path = Config::default_path()?;

    // Validate configuration
    output.info("🔍 Validating configuration...");
    match Config::load(None) {
        Ok(_) => output.success("✅ Configuration is valid"),
        Err(e) => {
            output.error(&format!("❌ Configuration validation failed: {}", e));
            return Err(e);
        }
    }

    // Perform backup if requested
    if backup {
        output.info("\n📦 Creating configuration backup...");

        if !config_path.exists() {
            output.warning("No configuration file found to backup");
        } else {
            let backup_dir = config_path
                .parent()
                .ok_or_else(|| CldevError::config("Invalid config path"))?
                .join("backups");

            fs::create_dir_all(&backup_dir)
                .map_err(|e| CldevError::io(format!("Failed to create backup directory: {}", e)))?;

            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let backup_path = backup_dir.join(format!("config.toml.{}", timestamp));

            fs::copy(&config_path, &backup_path)
                .map_err(|e| CldevError::io(format!("Failed to create backup: {}", e)))?;

            output.success(&format!("✅ Backup created: {}", backup_path.display()));
        }
    }

    // Cleanup old backups if requested
    if cleanup {
        output.info("\n🧹 Cleaning up old backups...");

        let backup_dir = config_path
            .parent()
            .ok_or_else(|| CldevError::config("Invalid config path"))?
            .join("backups");

        if !backup_dir.exists() {
            output.info("No backup directory found");
        } else {
            let mut backups: Vec<PathBuf> = fs::read_dir(&backup_dir)
                .map_err(|e| CldevError::io(format!("Failed to read backup directory: {}", e)))?
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| {
                    path.is_file()
                        && path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.starts_with("config.toml."))
                            .unwrap_or(false)
                })
                .collect();

            if backups.is_empty() {
                output.info("No backups found");
            } else {
                // Sort by modification time (newest first)
                backups.sort_by_key(|path| {
                    fs::metadata(path)
                        .and_then(|m| m.modified())
                        .ok()
                        .and_then(|t| std::time::SystemTime::now().duration_since(t).ok())
                });

                let keep_count = 10;
                let remove_count = backups.len().saturating_sub(keep_count);

                if remove_count > 0 {
                    output.info(&format!(
                        "Keeping {} most recent backups, removing {} old backups",
                        keep_count, remove_count
                    ));

                    for backup in backups.iter().skip(keep_count) {
                        match fs::remove_file(backup) {
                            Ok(_) => output.success(&format!(
                                "  Removed: {}",
                                backup.file_name().unwrap().to_string_lossy()
                            )),
                            Err(e) => output.warning(&format!(
                                "  Failed to remove {}: {}",
                                backup.display(),
                                e
                            )),
                        }
                    }
                } else {
                    output.info(&format!("Found {} backups (keeping all)", backups.len()));
                }
            }
        }
    }

    // Report configuration health
    output.info("\n📊 Configuration Health Report:");
    output.list_item(&format!("Config location: {}", config_path.display()));
    output.list_item(&format!("Config exists: {}", config_path.exists()));

    if config_path.exists() {
        if let Ok(metadata) = fs::metadata(&config_path) {
            output.list_item(&format!("Config size: {} bytes", metadata.len()));

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = metadata.permissions().mode();
                let mode_str = format!("{:o}", mode & 0o777);
                output.list_item(&format!("Permissions: {}", mode_str));
            }
        }
    }

    if !backup && !cleanup {
        output.info("\n💡 Tip: Use --backup to create a backup or --cleanup to remove old backups");
    }

    Ok(())
}
