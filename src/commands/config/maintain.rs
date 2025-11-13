use crate::cli::output::OutputHandler;
use crate::core::{
    config::Config, error::CldevError, learning_record_v3::LearningRecordV3, Result,
};
use chrono::{DateTime, Duration, Local};
use std::fs;
use std::path::PathBuf;

/// Archive configuration
#[derive(Debug, Clone)]
pub struct ArchiveConfig {
    pub retention_days: i64,
    #[allow(dead_code)]
    pub auto_archive: bool,
}

impl Default for ArchiveConfig {
    fn default() -> Self {
        Self {
            retention_days: 365, // 1 year retention by default
            auto_archive: false,
        }
    }
}

pub fn handle_config_maintain(
    backup: bool,
    cleanup: bool,
    archive: bool,
    retention_days: Option<i64>,
    output: &OutputHandler,
) -> Result<()> {
    let config_path = Config::default_path()?;

    // Validate configuration
    output.info(&output.t("config-maintain-validating"));
    match Config::load(None) {
        Ok(_) => output.success(&output.t("config-maintain-config-valid")),
        Err(e) => {
            output.error(&output.t_format(
                "config-maintain-validation-failed",
                "error",
                &e.to_string(),
            ));
            return Err(e);
        }
    }

    // Perform backup if requested
    if backup {
        output.info(&output.t("config-maintain-backing-up"));

        if !config_path.exists() {
            output.warning(&output.t("config-maintain-backup-none"));
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

            output.success(&output.t_format(
                "config-maintain-backup-created",
                "path",
                &backup_path.display().to_string(),
            ));
        }
    }

    // Cleanup old backups if requested
    if cleanup {
        output.info(&output.t("config-maintain-cleaning-up"));

        let backup_dir = config_path
            .parent()
            .ok_or_else(|| CldevError::config("Invalid config path"))?
            .join("backups");

        if !backup_dir.exists() {
            output.info(&output.t("config-maintain-dir-none"));
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
                output.info(&output.t("config-maintain-backups-none"));
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
                    output.info(
                        &output
                            .t_format(
                                "config-maintain-backups-removing",
                                "keep",
                                &keep_count.to_string(),
                            )
                            .replace("{remove}", &remove_count.to_string()),
                    );

                    for backup in backups.iter().skip(keep_count) {
                        match fs::remove_file(backup) {
                            Ok(_) => output.success(&output.t_format(
                                "config-maintain-remove-success",
                                "file",
                                &backup.file_name().unwrap().to_string_lossy(),
                            )),
                            Err(e) => output.warning(
                                &output
                                    .t_format(
                                        "config-maintain-remove-failed",
                                        "path",
                                        &backup.display().to_string(),
                                    )
                                    .replace("{error}", &e.to_string()),
                            ),
                        }
                    }
                } else {
                    output.info(&output.t_format(
                        "config-maintain-backups-found",
                        "count",
                        &backups.len().to_string(),
                    ));
                }
            }
        }
    }

    // Report configuration health
    output.info(&output.t("config-maintain-health-report"));
    output.list_item(&output.t_format(
        "config-maintain-config-location",
        "path",
        &config_path.display().to_string(),
    ));
    output.list_item(&output.t_format(
        "config-maintain-config-exists",
        "exists",
        &config_path.exists().to_string(),
    ));

    if config_path.exists() {
        if let Ok(metadata) = fs::metadata(&config_path) {
            output.list_item(&output.t_format(
                "config-maintain-config-size",
                "size",
                &metadata.len().to_string(),
            ));

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = metadata.permissions().mode();
                let mode_str = format!("{:o}", mode & 0o777);
                output.list_item(&output.t_format(
                    "config-maintain-config-permissions",
                    "permissions",
                    &mode_str,
                ));
            }
        }
    }

    // Perform learning records archive if requested
    if archive {
        output.info(&output.t("config-maintain-archiving"));
        let archive_config = ArchiveConfig {
            retention_days: retention_days.unwrap_or(365),
            auto_archive: true,
        };

        match archive_learning_records(&archive_config, output) {
            Ok(count) => {
                if count > 0 {
                    output.success(&output.t_format(
                        "config-maintain-archive-success",
                        "count",
                        &count.to_string(),
                    ));
                } else {
                    output.info(&output.t("config-maintain-archive-none"));
                }
            }
            Err(e) => {
                output.error(&output.t_format(
                    "config-maintain-archive-failed",
                    "error",
                    &e.to_string(),
                ));
                return Err(e);
            }
        }
    }

    if !backup && !cleanup && !archive {
        output.info(&output.t("config-maintain-tip"));
    }

    Ok(())
}

/// Detect expired learning records based on retention policy
fn detect_expired_records(retention_days: i64) -> Result<Vec<(String, DateTime<Local>)>> {
    let home =
        dirs::home_dir().ok_or_else(|| CldevError::config("Could not determine home directory"))?;

    let lr_dir = home.join(".cldev").join("learning-records");
    if !lr_dir.exists() {
        return Ok(Vec::new());
    }

    let cutoff_date = Local::now() - Duration::days(retention_days);
    let mut expired = Vec::new();

    for entry in fs::read_dir(&lr_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                // Try to load the record to get its creation date
                if let Ok(record) = LearningRecordV3::load(file_name) {
                    if record.created < cutoff_date {
                        expired.push((file_name.to_string(), record.created));
                    }
                }
            }
        }
    }

    // Sort by date (oldest first)
    expired.sort_by(|a, b| a.1.cmp(&b.1));

    Ok(expired)
}

/// Archive learning records
fn archive_learning_records(config: &ArchiveConfig, output: &OutputHandler) -> Result<usize> {
    let home =
        dirs::home_dir().ok_or_else(|| CldevError::config("Could not determine home directory"))?;

    let lr_dir = home.join(".cldev").join("learning-records");
    let archive_dir = home.join(".cldev").join("learning-records-archive");

    // Create archive directory if it doesn't exist
    if !archive_dir.exists() {
        fs::create_dir_all(&archive_dir)?;
    }

    // Detect expired records
    let expired_records = detect_expired_records(config.retention_days)?;

    if expired_records.is_empty() {
        return Ok(0);
    }

    output.info(
        &output
            .t_format(
                "config-maintain-archive-found",
                "count",
                &expired_records.len().to_string(),
            )
            .replace("{days}", &config.retention_days.to_string()),
    );

    // Create year-based subdirectory
    let current_year = Local::now().format("%Y").to_string();
    let year_dir = archive_dir.join(&current_year);
    if !year_dir.exists() {
        fs::create_dir_all(&year_dir)?;
    }

    // Create archive file
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let archive_path = year_dir.join(format!("archive_{}.tar.gz", timestamp));

    // Use tar and gzip to create compressed archive
    let tar_gz = fs::File::create(&archive_path)?;
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    let mut archived_count = 0;

    for (record_id, created_date) in &expired_records {
        let source_path = lr_dir.join(format!("{}.md", record_id));

        if source_path.exists() {
            // Add to tar archive
            let mut file = fs::File::open(&source_path)?;
            tar.append_file(format!("{}.md", record_id), &mut file)?;

            // Remove original file
            fs::remove_file(&source_path)?;

            output.list_item(
                &output
                    .t_format("config-maintain-archive-item", "id", record_id)
                    .replace("{date}", &created_date.format("%Y-%m-%d").to_string()),
            );

            archived_count += 1;
        }
    }

    tar.finish()?;

    output.success(&output.t_format(
        "config-maintain-archive-created",
        "path",
        &archive_path.display().to_string(),
    ));

    Ok(archived_count)
}
