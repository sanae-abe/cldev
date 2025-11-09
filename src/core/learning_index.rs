//! Learning Index V2 - Fast in-memory index for learning records
//!
//! Provides keyword-based search and hotspot tracking.
//! Backed by SQLite for persistence (via LearningDatabase).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Index Types
// ============================================================================

/// Session reference for search results
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRef {
    pub id: String,
    pub session_type: String,
    pub timestamp: String,
    pub title: String,
}

/// File hotspot information
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHotspot {
    pub issue_count: usize,
    pub avg_hotspot_score: f64,
    pub hotspot_level: String,
    pub last_affected: String,
}

/// Timeline entry
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub id: String,
    pub timestamp: String,
    pub session_type: String,
    pub title: String,
    pub resolved: bool,
}

/// Unresolved entry
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedEntry {
    pub id: String,
    pub priority: String,
    pub title: String,
    pub days_open: u32,
}

/// Main index structure
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningIndexV2 {
    pub total_records: usize,
    pub keywords: HashMap<String, Vec<String>>,
    pub tags: HashMap<String, Vec<String>>,
    pub file_hotspots: HashMap<String, FileHotspot>,
    pub timeline: Vec<TimelineEntry>,
    pub last_rebuild: String,
}

impl Default for LearningIndexV2 {
    fn default() -> Self {
        Self {
            total_records: 0,
            keywords: HashMap::new(),
            tags: HashMap::new(),
            file_hotspots: HashMap::new(),
            timeline: Vec::new(),
            last_rebuild: chrono::Local::now().to_rfc3339(),
        }
    }
}

impl LearningIndexV2 {
    /// Get index file path
    fn index_path() -> crate::core::Result<std::path::PathBuf> {
        use crate::core::CldevError;
        let home = std::env::var("HOME")
            .ok()
            .map(std::path::PathBuf::from)
            .or_else(dirs::home_dir)
            .ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let index_dir = home.join(".claude").join("learning-records");
        if !index_dir.exists() {
            std::fs::create_dir_all(&index_dir)?;
        }

        Ok(index_dir.join(".index.json"))
    }

    /// Load index from JSON file
    #[allow(dead_code)]
    pub fn load() -> crate::core::Result<Self> {
        use crate::core::CldevError;
        let path = Self::index_path()?;
        if !path.exists() {
            Self::rebuild()?;
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)?;
        let index: Self = serde_json::from_str(&content)
            .map_err(|e| CldevError::config(format!("Failed to parse index: {}", e)))?;

        Ok(index)
    }

    /// Save index to JSON file
    fn save(&self) -> crate::core::Result<()> {
        use crate::core::CldevError;
        let path = Self::index_path()?;
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| CldevError::config(format!("Failed to serialize index: {}", e)))?;

        std::fs::write(path, content)?;
        Ok(())
    }

    /// Rebuild index using LearningDatabase
    pub fn rebuild() -> crate::core::Result<()> {
        use crate::core::{CldevError, LearningDatabase};
        use std::path::PathBuf;

        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
            .ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let db_path = home
            .join(".claude")
            .join("learning-records")
            .join("learning.db");
        let markdown_dir = home.join(".claude").join("learning-records");

        let mut db = LearningDatabase::new(db_path, markdown_dir)?;
        let (inserted, updated) = db.build_from_markdown()?;

        // Create minimal index file for compatibility
        let index = Self::default();
        index.save()?;

        println!("Index rebuilt: {} inserted, {} updated", inserted, updated);
        Ok(())
    }

    /// Find sessions by keyword (delegates to database)
    #[allow(dead_code)]
    pub fn find_by_keyword(&self, keyword: &str) -> Vec<String> {
        use crate::core::LearningDatabase;
        use std::path::PathBuf;

        let home = match std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
        {
            Some(h) => h,
            None => return Vec::new(),
        };

        let db_path = home
            .join(".claude")
            .join("learning-records")
            .join("learning.db");
        let markdown_dir = home.join(".claude").join("learning-records");

        let db = match LearningDatabase::new(db_path, markdown_dir) {
            Ok(db) => db,
            Err(_) => return Vec::new(),
        };

        match db.query_by_keyword(keyword, 100) {
            Ok(results) => results.iter().map(|r| r.session.id.clone()).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Find sessions by tag (delegates to database)
    #[allow(dead_code)]
    pub fn find_by_tag(&self, tag: &str) -> Vec<String> {
        use crate::core::LearningDatabase;
        use std::path::PathBuf;

        let home = match std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
        {
            Some(h) => h,
            None => return Vec::new(),
        };

        let db_path = home
            .join(".claude")
            .join("learning-records")
            .join("learning.db");
        let markdown_dir = home.join(".claude").join("learning-records");

        let db = match LearningDatabase::new(db_path, markdown_dir) {
            Ok(db) => db,
            Err(_) => return Vec::new(),
        };

        match db.query_by_tag(tag, 100) {
            Ok(results) => results.iter().map(|r| r.session.id.clone()).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Get file hotspots (delegates to database)
    #[allow(dead_code)]
    pub fn get_hotspots(&self, min_issues: usize) -> Vec<(String, FileHotspot)> {
        use crate::core::LearningDatabase;
        use std::path::PathBuf;

        let home = match std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
        {
            Some(h) => h,
            None => return Vec::new(),
        };

        let db_path = home
            .join(".claude")
            .join("learning-records")
            .join("learning.db");
        let markdown_dir = home.join(".claude").join("learning-records");

        let db = match LearningDatabase::new(db_path, markdown_dir) {
            Ok(db) => db,
            Err(_) => return Vec::new(),
        };

        match db.get_hotspots(100) {
            Ok(hotspots) => hotspots
                .iter()
                .filter(|h| h.session_count >= min_issues)
                .map(|h| {
                    let level = if h.session_count >= 5 {
                        "critical"
                    } else if h.session_count >= 3 {
                        "high"
                    } else {
                        "medium"
                    };

                    (
                        h.file_path.clone(),
                        FileHotspot {
                            issue_count: h.session_count,
                            avg_hotspot_score: h.avg_hotspot_score,
                            hotspot_level: level.to_string(),
                            last_affected: h.last_accessed.clone(),
                        },
                    )
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_index() {
        let index = LearningIndexV2::default();
        assert_eq!(index.total_records, 0);
    }
}
