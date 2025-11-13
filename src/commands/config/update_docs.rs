use std::path::Path;

use crate::cli::args::DocType;
use crate::cli::output::OutputHandler;
use crate::core::Result;

/// Handle config update-docs command
pub fn handle_update_docs(
    doc_type: Option<&DocType>,
    validate: bool,
    output: &OutputHandler,
) -> Result<()> {
    // If no doc type specified, show available options
    if doc_type.is_none() {
        output.section(&output.t("config-update-docs-available"));
        output.list_item(&output.t("config-update-docs-type-impl"));
        output.list_item(&output.t("config-update-docs-type-api"));
        output.list_item(&output.t("config-update-docs-type-arch"));
        output.raw("");
        output.info(&output.t("config-update-docs-usage"));
        output.info(&output.t("config-update-docs-usage-validate"));
        return Ok(());
    }

    let doc_type = doc_type.unwrap();

    // Validation mode
    if validate {
        output.section(&output.t("config-update-docs-validating"));

        let docs_dir = Path::new("docs");
        if !docs_dir.exists() {
            output.warning(&output.t("config-update-docs-location"));
            output.info(&output.t("config-update-docs-structure-consider"));
            output.list_item(&output.t("config-update-docs-structure-impl"));
            output.list_item(&output.t("config-update-docs-structure-api"));
            output.list_item(&output.t("config-update-docs-structure-arch"));
            return Ok(());
        }

        // Count markdown files
        let mut total_docs = 0;
        if let Ok(entries) = std::fs::read_dir(docs_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "md" {
                                total_docs += 1;
                            }
                        }
                    } else if file_type.is_dir() {
                        // Count files in subdirectories
                        if let Ok(sub_entries) = std::fs::read_dir(entry.path()) {
                            for sub_entry in sub_entries.flatten() {
                                if let Some(ext) = sub_entry.path().extension() {
                                    if ext == "md" {
                                        total_docs += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        output.success(&output.t_format(
            "config-update-docs-found",
            "count",
            &total_docs.to_string(),
        ));
        output.info(&output.t("config-update-docs-structure"));
        output.list_item(&output.t_format(
            "config-update-docs-structure-location",
            "path",
            &docs_dir.display().to_string(),
        ));
        output.list_item(&output.t_format(
            "config-update-docs-structure-total",
            "count",
            &total_docs.to_string(),
        ));
    }

    // Show doc type specific message
    match doc_type {
        DocType::Implementation => {
            output.section(&output.t("config-update-docs-impl-title"));
            output.info(&output.t("config-update-docs-impl-desc"));
            output.list_item(&output.t("config-update-docs-impl-scan"));
            output.list_item(&output.t("config-update-docs-impl-extract"));
            output.list_item(&output.t("config-update-docs-impl-examples"));
            output.list_item(&output.t("config-update-docs-impl-guides"));
        }
        DocType::Api => {
            output.section(&output.t("config-update-docs-api-title"));
            output.info(&output.t("config-update-docs-api-desc"));
            output.list_item(&output.t("config-update-docs-api-endpoints"));
            output.list_item(&output.t("config-update-docs-api-schemas"));
            output.list_item(&output.t("config-update-docs-api-examples"));
            output.list_item(&output.t("config-update-docs-api-reference"));
        }
        DocType::Architecture => {
            output.section(&output.t("config-update-docs-arch-title"));
            output.info(&output.t("config-update-docs-arch-desc"));
            output.list_item(&output.t("config-update-docs-arch-structure"));
            output.list_item(&output.t("config-update-docs-arch-diagrams"));
            output.list_item(&output.t("config-update-docs-arch-patterns"));
            output.list_item(&output.t("config-update-docs-arch-guides"));
        }
    }

    output.raw("");
    output.warning(&output.t("config-update-docs-coming-soon"));
    output.info(&output.t("config-update-docs-output-dir"));

    Ok(())
}
