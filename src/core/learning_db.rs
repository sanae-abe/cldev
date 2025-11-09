//! SQLite-based Learning Database
//!
//! Hybrid approach: Markdown files for human-readable storage + SQLite for fast queries
//! - Full-text search (FTS5) for content queries
//! - Indexed lookups for files, tags, errors
//! - Hotspot scoring and prioritization
//! - Staleness detection via mtime tracking

use crate::core::learning_record_v2::{LearningRecordV2, Priority};
use crate::core::similarity::calculate_similarity;
use crate::core::tfidf::{TfidfIndex, TfidfResult};
use crate::core::{CldevError, Result};
use chrono::{DateTime, Local, TimeZone};
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Learning database with SQLite backend
pub struct LearningDatabase {
    conn: Connection,
    #[allow(dead_code)]
    db_path: PathBuf,
    #[allow(dead_code)]
    markdown_dir: PathBuf,
    #[allow(dead_code)]
    tfidf_index: TfidfIndex,
}

/// Session metadata stored in DB
#[derive(Debug, Clone)]
pub struct SessionMetadata {
    pub id: String,
    pub session_type: String,
    pub priority: String,
    pub timestamp: String,
    pub resolved: bool,
    #[allow(dead_code)]
    pub duration_minutes: Option<i64>,
    pub title: String,
    pub description: String,
    pub markdown_path: String,
    #[allow(dead_code)]
    pub hotspot_score: f64,
    #[allow(dead_code)]
    pub created_at: String,
    #[allow(dead_code)]
    pub updated_at: String,
}

/// Query result with session metadata and matched files
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub session: SessionMetadata,
    pub matched_files: Vec<String>,
    pub matched_tags: Vec<String>,
    pub relevance_score: f64,
}

/// Hotspot entry for frequently accessed files
#[derive(Debug, Clone)]
pub struct Hotspot {
    pub file_path: String,
    pub session_count: usize,
    pub avg_hotspot_score: f64,
    pub last_accessed: String,
}

impl LearningDatabase {
    /// Create or open a learning database
    pub fn new(db_path: PathBuf, markdown_dir: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path).map_err(|e| {
            CldevError::Config(format!("Failed to open database at {:?}: {}", db_path, e))
        })?;

        let mut db = Self {
            conn,
            db_path,
            markdown_dir,
            tfidf_index: TfidfIndex::new(),
        };

        db.initialize_schema()?;
        Ok(db)
    }

    /// Initialize database schema with FTS5 support
    fn initialize_schema(&mut self) -> Result<()> {
        self.conn
            .execute_batch(
                r#"
            -- Main sessions table
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                session_type TEXT NOT NULL,
                priority TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                resolved BOOLEAN NOT NULL DEFAULT 0,
                duration_minutes INTEGER,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                markdown_path TEXT NOT NULL,
                markdown_mtime INTEGER NOT NULL,
                hotspot_score REAL NOT NULL DEFAULT 0.0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_type ON sessions(session_type);
            CREATE INDEX IF NOT EXISTS idx_sessions_priority ON sessions(priority);
            CREATE INDEX IF NOT EXISTS idx_sessions_resolved ON sessions(resolved);
            CREATE INDEX IF NOT EXISTS idx_sessions_hotspot ON sessions(hotspot_score DESC);
            CREATE INDEX IF NOT EXISTS idx_sessions_timestamp ON sessions(timestamp DESC);

            -- Full-text search index
            CREATE VIRTUAL TABLE IF NOT EXISTS sessions_fts USING fts5(
                id UNINDEXED,
                title,
                description,
                tags,
                error_patterns,
                content='',
                tokenize='porter unicode61'
            );

            -- Files table
            CREATE TABLE IF NOT EXISTS files (
                session_id TEXT NOT NULL,
                file_path TEXT NOT NULL,
                role TEXT NOT NULL,
                hotspot_score REAL NOT NULL DEFAULT 0.0,
                PRIMARY KEY (session_id, file_path),
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_files_path ON files(file_path);
            CREATE INDEX IF NOT EXISTS idx_files_hotspot ON files(hotspot_score DESC);

            -- Tags table
            CREATE TABLE IF NOT EXISTS tags (
                session_id TEXT NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY (session_id, tag),
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_tags_tag ON tags(tag);

            -- Errors table
            CREATE TABLE IF NOT EXISTS errors (
                session_id TEXT NOT NULL,
                error_pattern TEXT NOT NULL,
                stack_trace_hash TEXT,
                PRIMARY KEY (session_id, error_pattern),
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_errors_pattern ON errors(error_pattern);
            CREATE INDEX IF NOT EXISTS idx_errors_hash ON errors(stack_trace_hash);
        "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to initialize schema: {}", e)))?;

        Ok(())
    }

    /// Build index from all markdown files in the directory
    pub fn build_from_markdown(&mut self) -> Result<(usize, usize)> {
        let mut inserted = 0;
        let mut updated = 0;

        if !self.markdown_dir.exists() {
            fs::create_dir_all(&self.markdown_dir).map_err(|e| {
                CldevError::Config(format!(
                    "Failed to create markdown directory {:?}: {}",
                    self.markdown_dir, e
                ))
            })?;
            return Ok((0, 0));
        }

        for entry in fs::read_dir(&self.markdown_dir).map_err(|e| {
            CldevError::Config(format!(
                "Failed to read markdown directory {:?}: {}",
                self.markdown_dir, e
            ))
        })? {
            let entry = entry.map_err(|e| {
                CldevError::Config(format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            let content = fs::read_to_string(&path).map_err(|e| {
                CldevError::Config(format!("Failed to read markdown file {:?}: {}", path, e))
            })?;

            // Parse frontmatter YAML
            if let Some(record) = Self::parse_markdown(&content)? {
                let was_updated =
                    self.upsert_session(&record, path.to_string_lossy().to_string())?;

                if was_updated {
                    updated += 1;
                } else {
                    inserted += 1;
                }
            }
        }

        Ok((inserted, updated))
    }

    /// Parse markdown file with YAML frontmatter
    fn parse_markdown(content: &str) -> Result<Option<LearningRecordV2>> {
        // Extract YAML frontmatter between --- delimiters
        if !content.starts_with("---\n") {
            return Ok(None);
        }

        let parts: Vec<&str> = content.splitn(3, "---\n").collect();
        if parts.len() < 3 {
            return Ok(None);
        }

        let yaml_content = parts[1];
        let record: LearningRecordV2 = serde_yaml::from_str(yaml_content)
            .map_err(|e| CldevError::Config(format!("Failed to parse YAML frontmatter: {}", e)))?;

        Ok(Some(record))
    }

    /// Get file modification time as Unix timestamp
    fn get_mtime(path: &Path) -> Result<i64> {
        let metadata = fs::metadata(path)
            .map_err(|e| CldevError::Config(format!("Failed to get metadata: {}", e)))?;

        let mtime = metadata
            .modified()
            .map_err(|e| CldevError::Config(format!("Failed to get mtime: {}", e)))?
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| CldevError::Config(format!("Invalid mtime: {}", e)))?
            .as_secs() as i64;

        Ok(mtime)
    }

    /// Upsert a session record (insert or update if exists)
    pub fn upsert_session(
        &mut self,
        record: &LearningRecordV2,
        markdown_path: String,
    ) -> Result<bool> {
        let mtime = Self::get_mtime(Path::new(&markdown_path))?;

        let session_id = &record.session_meta.id;
        let hotspot_score = self.calculate_hotspot_score(record);
        let now = Local::now().to_rfc3339();

        // Begin transaction for atomicity
        let tx = self
            .conn
            .transaction()
            .map_err(|e| CldevError::Config(format!("Failed to start transaction: {}", e)))?;

        // Check if session exists
        let exists: bool = tx
            .query_row(
                "SELECT 1 FROM sessions WHERE id = ?1",
                params![session_id],
                |_| Ok(true),
            )
            .optional()
            .map_err(|e| CldevError::Config(format!("Failed to check session existence: {}", e)))?
            .unwrap_or(false);

        // Upsert session
        tx.execute(
            r#"
            INSERT INTO sessions (
                id, session_type, priority, timestamp, resolved, duration_minutes,
                title, description, markdown_path, markdown_mtime, hotspot_score,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12)
            ON CONFLICT(id) DO UPDATE SET
                session_type = excluded.session_type,
                priority = excluded.priority,
                timestamp = excluded.timestamp,
                resolved = excluded.resolved,
                duration_minutes = excluded.duration_minutes,
                title = excluded.title,
                description = excluded.description,
                markdown_path = excluded.markdown_path,
                markdown_mtime = excluded.markdown_mtime,
                hotspot_score = excluded.hotspot_score,
                updated_at = excluded.updated_at
            "#,
            params![
                session_id,
                record.session_meta.session_type.as_str(),
                format!("{:?}", record.session_meta.priority).to_lowercase(),
                record.session_meta.timestamp.to_rfc3339(),
                record.session_meta.resolved,
                record.session_meta.duration_minutes,
                record.problem.title,
                record.problem.description,
                markdown_path,
                mtime,
                hotspot_score,
                now,
            ],
        )
        .map_err(|e| CldevError::Config(format!("Failed to upsert session: {}", e)))?;

        // Delete existing related data
        tx.execute(
            "DELETE FROM files WHERE session_id = ?1",
            params![session_id],
        )
        .map_err(|e| CldevError::Config(format!("Failed to delete files: {}", e)))?;
        tx.execute(
            "DELETE FROM tags WHERE session_id = ?1",
            params![session_id],
        )
        .map_err(|e| CldevError::Config(format!("Failed to delete tags: {}", e)))?;
        tx.execute(
            "DELETE FROM errors WHERE session_id = ?1",
            params![session_id],
        )
        .map_err(|e| CldevError::Config(format!("Failed to delete errors: {}", e)))?;
        tx.execute(
            "DELETE FROM sessions_fts WHERE id = ?1",
            params![session_id],
        )
        .map_err(|e| CldevError::Config(format!("Failed to delete FTS entry: {}", e)))?;

        // Insert files
        for file in &record.context.files_affected {
            tx.execute(
                "INSERT INTO files (session_id, file_path, role, hotspot_score) VALUES (?1, ?2, ?3, ?4)",
                params![
                    session_id,
                    &file.path,
                    format!("{:?}", file.role).to_lowercase(),
                    file.hotspot_score,
                ],
            )
            .map_err(|e| CldevError::Config(format!("Failed to insert file: {}", e)))?;
        }

        // Insert tags
        for tag in &record.context.tags {
            tx.execute(
                "INSERT INTO tags (session_id, tag) VALUES (?1, ?2)",
                params![session_id, tag],
            )
            .map_err(|e| CldevError::Config(format!("Failed to insert tag: {}", e)))?;
        }

        // Insert error patterns
        for error in &record.problem.error_signatures {
            tx.execute(
                "INSERT INTO errors (session_id, error_pattern, stack_trace_hash) VALUES (?1, ?2, ?3)",
                params![session_id, &error.pattern, &error.stack_trace_hash],
            )
            .map_err(|e| CldevError::Config(format!("Failed to insert error: {}", e)))?;
        }

        // Insert FTS entry
        let tags_str = record.context.tags.join(" ");
        let errors_str = record
            .problem
            .error_signatures
            .iter()
            .map(|e| e.pattern.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        tx.execute(
            "INSERT INTO sessions_fts (id, title, description, tags, error_patterns) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                session_id,
                &record.problem.title,
                &record.problem.description,
                &tags_str,
                &errors_str,
            ],
        )
        .map_err(|e| CldevError::Config(format!("Failed to insert FTS entry: {}", e)))?;

        // Commit transaction
        tx.commit()
            .map_err(|e| CldevError::Config(format!("Failed to commit transaction: {}", e)))?;

        // Update TF-IDF index
        // Build searchable text from title, description, tags, and error patterns
        let searchable_text = format!(
            "{} {} {} {}",
            record.problem.title, record.problem.description, tags_str, errors_str
        );

        // Remove old entry if exists
        if exists {
            self.tfidf_index.remove_document(session_id);
        }

        // Add new entry
        self.tfidf_index.add_document(session_id, &searchable_text);

        Ok(exists)
    }

    /// Calculate hotspot score based on priority, recency, and file counts
    fn calculate_hotspot_score(&self, record: &LearningRecordV2) -> f64 {
        let priority_weight = match record.session_meta.priority {
            Priority::Critical => 10.0,
            Priority::High => 7.0,
            Priority::Medium => 4.0,
            Priority::Low => 1.0,
        };

        let recency_weight = {
            let age_days = (Local::now() - record.session_meta.timestamp).num_days();
            (1.0 / (1.0 + age_days as f64 * 0.1)).max(0.1)
        };

        let file_weight = (record.context.files_affected.len() as f64).min(10.0);

        let unresolved_weight = if record.session_meta.resolved {
            1.0
        } else {
            2.0
        };

        priority_weight * recency_weight * (1.0 + file_weight * 0.1) * unresolved_weight
    }

    /// Query sessions by keyword (full-text search)
    pub fn query_by_keyword(&self, keyword: &str, limit: usize) -> Result<Vec<QueryResult>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
            SELECT DISTINCT s.*, fts.rank
            FROM sessions_fts fts
            JOIN sessions s ON fts.id = s.id
            WHERE sessions_fts MATCH ?1
            ORDER BY fts.rank, s.hotspot_score DESC
            LIMIT ?2
            "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let results = stmt
            .query_map(params![keyword, limit], |row| {
                Ok((
                    Self::row_to_session_metadata(row)?,
                    row.get::<_, f64>(12)?, // rank
                ))
            })
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect results: {}", e)))?;

        self.enhance_query_results(results)
    }

    /// Query sessions by file path
    pub fn query_by_file(&self, file_path: &str, limit: usize) -> Result<Vec<QueryResult>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
            SELECT DISTINCT s.*, f.hotspot_score
            FROM files f
            JOIN sessions s ON f.session_id = s.id
            WHERE f.file_path LIKE ?1
            ORDER BY f.hotspot_score DESC, s.hotspot_score DESC
            LIMIT ?2
            "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let pattern = format!("%{}%", file_path);
        let results = stmt
            .query_map(params![pattern, limit], |row| {
                Ok((
                    Self::row_to_session_metadata(row)?,
                    row.get::<_, f64>(12)?, // file hotspot_score
                ))
            })
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect results: {}", e)))?;

        self.enhance_query_results(results)
    }

    /// Query sessions by tag
    pub fn query_by_tag(&self, tag: &str, limit: usize) -> Result<Vec<QueryResult>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
            SELECT DISTINCT s.*, 1.0 as score
            FROM tags t
            JOIN sessions s ON t.session_id = s.id
            WHERE t.tag = ?1
            ORDER BY s.hotspot_score DESC
            LIMIT ?2
            "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let results = stmt
            .query_map(params![tag, limit], |row| {
                Ok((Self::row_to_session_metadata(row)?, 1.0))
            })
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect results: {}", e)))?;

        self.enhance_query_results(results)
    }

    /// Query sessions by error pattern
    #[allow(dead_code)]
    pub fn query_by_error(&self, error_pattern: &str, limit: usize) -> Result<Vec<QueryResult>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
            SELECT DISTINCT s.*, 1.0 as score
            FROM errors e
            JOIN sessions s ON e.session_id = s.id
            WHERE e.error_pattern LIKE ?1
            ORDER BY s.hotspot_score DESC
            LIMIT ?2
            "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let pattern = format!("%{}%", error_pattern);
        let results = stmt
            .query_map(params![pattern, limit], |row| {
                Ok((Self::row_to_session_metadata(row)?, 1.0))
            })
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect results: {}", e)))?;

        self.enhance_query_results(results)
    }

    /// Get file hotspots (most frequently accessed files)
    pub fn get_hotspots(&self, limit: usize) -> Result<Vec<Hotspot>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
            SELECT
                f.file_path,
                COUNT(DISTINCT f.session_id) as session_count,
                AVG(f.hotspot_score) as avg_score,
                MAX(s.timestamp) as last_accessed
            FROM files f
            JOIN sessions s ON f.session_id = s.id
            GROUP BY f.file_path
            ORDER BY avg_score DESC, session_count DESC
            LIMIT ?1
            "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let hotspots = stmt
            .query_map(params![limit], |row| {
                Ok(Hotspot {
                    file_path: row.get(0)?,
                    session_count: row.get(1)?,
                    avg_hotspot_score: row.get(2)?,
                    last_accessed: row.get(3)?,
                })
            })
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect hotspots: {}", e)))?;

        Ok(hotspots)
    }

    /// Get unresolved sessions
    #[allow(dead_code)]
    pub fn get_unresolved(&self, limit: usize) -> Result<Vec<QueryResult>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
            SELECT *, 1.0 as score
            FROM sessions
            WHERE resolved = 0
            ORDER BY hotspot_score DESC, timestamp DESC
            LIMIT ?1
            "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let results = stmt
            .query_map(params![limit], |row| {
                Ok((Self::row_to_session_metadata(row)?, 1.0))
            })
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect results: {}", e)))?;

        self.enhance_query_results(results)
    }

    /// Check if the database is stale (markdown files modified after last index)
    #[allow(dead_code)]
    pub fn is_stale(&self) -> Result<bool> {
        if !self.markdown_dir.exists() {
            return Ok(false);
        }

        for entry in fs::read_dir(&self.markdown_dir)
            .map_err(|e| CldevError::Config(format!("Failed to read markdown directory: {}", e)))?
        {
            let entry = entry.map_err(|e| {
                CldevError::Config(format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            let mtime = Self::get_mtime(&path)?;
            let path_str = path.to_string_lossy().to_string();

            // Check if DB has this file and if it's up-to-date
            let db_mtime: Option<i64> = self
                .conn
                .query_row(
                    "SELECT markdown_mtime FROM sessions WHERE markdown_path = ?1",
                    params![path_str],
                    |row| row.get(0),
                )
                .optional()
                .map_err(|e| CldevError::Config(format!("Failed to check mtime: {}", e)))?;

            match db_mtime {
                None => return Ok(true),                               // New file not in DB
                Some(db_mtime) if db_mtime < mtime => return Ok(true), // File modified
                _ => {}
            }
        }

        Ok(false)
    }

    /// Delete a session by ID
    #[allow(dead_code)]
    pub fn delete_session(&mut self, session_id: &str) -> Result<bool> {
        let deleted = self
            .conn
            .execute("DELETE FROM sessions WHERE id = ?1", params![session_id])
            .map_err(|e| CldevError::Config(format!("Failed to delete session: {}", e)))?;

        // Remove from TF-IDF index
        if deleted > 0 {
            self.tfidf_index.remove_document(session_id);
        }

        Ok(deleted > 0)
    }

    /// Helper: Convert row to SessionMetadata
    fn row_to_session_metadata(row: &rusqlite::Row) -> rusqlite::Result<SessionMetadata> {
        Ok(SessionMetadata {
            id: row.get(0)?,
            session_type: row.get(1)?,
            priority: row.get(2)?,
            timestamp: row.get(3)?,
            resolved: row.get(4)?,
            duration_minutes: row.get(5)?,
            title: row.get(6)?,
            description: row.get(7)?,
            markdown_path: row.get(8)?,
            hotspot_score: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    }

    /// Helper: Enhance query results with files and tags
    fn enhance_query_results(
        &self,
        results: Vec<(SessionMetadata, f64)>,
    ) -> Result<Vec<QueryResult>> {
        results
            .into_iter()
            .map(|(session, relevance_score)| {
                let matched_files = self.get_session_files(&session.id)?;
                let matched_tags = self.get_session_tags(&session.id)?;

                Ok(QueryResult {
                    session,
                    matched_files,
                    matched_tags,
                    relevance_score,
                })
            })
            .collect()
    }

    /// Get files for a session
    fn get_session_files(&self, session_id: &str) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT file_path FROM files WHERE session_id = ?1")
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let files = stmt
            .query_map(params![session_id], |row| row.get(0))
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect files: {}", e)))?;

        Ok(files)
    }

    /// Get tags for a session
    fn get_session_tags(&self, session_id: &str) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT tag FROM tags WHERE session_id = ?1")
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let tags = stmt
            .query_map(params![session_id], |row| row.get(0))
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect tags: {}", e)))?;

        Ok(tags)
    }
    /// Suggest sessions by context with composite scoring
    ///
    /// # Scoring weights
    /// Search sessions using TF-IDF ranking
    ///
    /// This provides keyword-based relevance ranking that complements the FTS5 full-text search.
    /// TF-IDF scoring considers both term frequency (how often a term appears in a document)
    /// and inverse document frequency (how rare a term is across all documents).
    ///
    /// # Arguments
    /// * `query` - Search query text
    /// * `limit` - Maximum number of results
    ///
    /// # Returns
    /// Query results sorted by TF-IDF score (most relevant first)
    #[allow(dead_code)]
    pub fn search_with_tfidf(&self, query: &str, limit: usize) -> Result<Vec<QueryResult>> {
        let tfidf_results = self.tfidf_index.search(query, limit);

        // Convert TF-IDF results to QueryResults by fetching session metadata
        let mut results = Vec::new();

        for tfidf_result in tfidf_results {
            // Fetch session metadata from database
            let session_opt = self
                .conn
                .query_row(
                    "SELECT * FROM sessions WHERE id = ?1",
                    params![&tfidf_result.doc_id],
                    Self::row_to_session_metadata,
                )
                .optional()
                .map_err(|e| {
                    CldevError::Config(format!("Failed to fetch session metadata: {}", e))
                })?;

            if let Some(session) = session_opt {
                let matched_files = self.get_session_files(&session.id)?;
                let matched_tags = self.get_session_tags(&session.id)?;

                results.push(QueryResult {
                    session,
                    matched_files,
                    matched_tags,
                    relevance_score: tfidf_result.score,
                });
            }
        }

        Ok(results)
    }

    /// Get TF-IDF index statistics
    ///
    /// Returns statistics about the TF-IDF index including document count,
    /// term count, and average document length.
    #[allow(dead_code)]
    pub fn tfidf_stats(&self) -> crate::core::tfidf::IndexStats {
        self.tfidf_index.stats()
    }
    /// - File match: 40%
    /// - Error similarity: 30%
    /// - Tag match: 20%
    /// - Recency: 10%
    ///
    /// # Parameters
    /// - `file_path`: Optional file path to match
    /// - `error_pattern`: Optional error pattern to match
    /// - `tags`: Optional tags to match
    /// - `limit`: Maximum number of results
    pub fn suggest_by_context(
        &self,
        file_path: Option<&str>,
        error_pattern: Option<&str>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<Vec<QueryResult>> {
        // Collect all candidate sessions with their partial scores
        let mut candidates: HashMap<String, CompositeScore> = HashMap::new();

        // Weight constants
        const FILE_WEIGHT: f64 = 0.4;
        const ERROR_WEIGHT: f64 = 0.3;
        const TAG_WEIGHT: f64 = 0.2;
        const RECENCY_WEIGHT: f64 = 0.1;

        // Calculate max days for recency normalization (365 days)
        const MAX_DAYS: f64 = 365.0;

        // 1. File matching (40%)
        if let Some(file) = file_path {
            let pattern = format!("%{}%", file);
            let mut stmt = self
                .conn
                .prepare(
                    r#"
                SELECT DISTINCT s.id, s.timestamp
                FROM files f
                JOIN sessions s ON f.session_id = s.id
                WHERE f.file_path LIKE ?1
                "#,
                )
                .map_err(|e| CldevError::Config(format!("Failed to prepare file query: {}", e)))?;

            let file_matches = stmt
                .query_map(params![pattern], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })
                .map_err(|e| CldevError::Config(format!("Failed to execute file query: {}", e)))?
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| {
                    CldevError::Config(format!("Failed to collect file results: {}", e))
                })?;

            for (session_id, timestamp) in file_matches {
                candidates
                    .entry(session_id)
                    .or_insert_with(|| CompositeScore {
                        session_id: String::new(),
                        timestamp: timestamp.clone(),
                        file_score: 0.0,
                        error_score: 0.0,
                        tag_score: 0.0,
                        recency_score: 0.0,
                    })
                    .file_score = FILE_WEIGHT;
            }
        }

        // 2. Error pattern matching (30%)
        if let Some(error) = error_pattern {
            let pattern = format!("%{}%", error);
            let mut stmt = self
                .conn
                .prepare(
                    r#"
                SELECT DISTINCT s.id, s.timestamp, e.error_pattern
                FROM errors e
                JOIN sessions s ON e.session_id = s.id
                WHERE e.error_pattern LIKE ?1
                "#,
                )
                .map_err(|e| CldevError::Config(format!("Failed to prepare error query: {}", e)))?;

            let error_matches = stmt
                .query_map(params![pattern], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                })
                .map_err(|e| CldevError::Config(format!("Failed to execute error query: {}", e)))?
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| {
                    CldevError::Config(format!("Failed to collect error results: {}", e))
                })?;

            for (session_id, timestamp, stored_pattern) in error_matches {
                let similarity = Self::calculate_error_similarity(error, &stored_pattern);
                candidates
                    .entry(session_id)
                    .or_insert_with(|| CompositeScore {
                        session_id: String::new(),
                        timestamp: timestamp.clone(),
                        file_score: 0.0,
                        error_score: 0.0,
                        tag_score: 0.0,
                        recency_score: 0.0,
                    })
                    .error_score = similarity * ERROR_WEIGHT;
            }
        }

        // 3. Tag matching (20%)
        if let Some(query_tags) = tags {
            if !query_tags.is_empty() {
                let placeholders = query_tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                let query = format!(
                    r#"
                    SELECT s.id, s.timestamp, COUNT(DISTINCT t.tag) as matching_tags
                    FROM tags t
                    JOIN sessions s ON t.session_id = s.id
                    WHERE t.tag IN ({})
                    GROUP BY s.id, s.timestamp
                    "#,
                    placeholders
                );

                let mut stmt = self.conn.prepare(&query).map_err(|e| {
                    CldevError::Config(format!("Failed to prepare tag query: {}", e))
                })?;

                let params_vec: Vec<&dyn rusqlite::ToSql> = query_tags
                    .iter()
                    .map(|t| t as &dyn rusqlite::ToSql)
                    .collect();

                let tag_matches = stmt
                    .query_map(params_vec.as_slice(), |row| {
                        Ok((
                            row.get::<_, String>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, usize>(2)?,
                        ))
                    })
                    .map_err(|e| CldevError::Config(format!("Failed to execute tag query: {}", e)))?
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(|e| {
                        CldevError::Config(format!("Failed to collect tag results: {}", e))
                    })?;

                for (session_id, timestamp, matching_count) in tag_matches {
                    let tag_ratio = matching_count as f64 / query_tags.len() as f64;
                    candidates
                        .entry(session_id)
                        .or_insert_with(|| CompositeScore {
                            session_id: String::new(),
                            timestamp: timestamp.clone(),
                            file_score: 0.0,
                            error_score: 0.0,
                            tag_score: 0.0,
                            recency_score: 0.0,
                        })
                        .tag_score = tag_ratio * TAG_WEIGHT;
                }
            }
        }

        // 4. Calculate recency scores (10%) for all candidates
        let now = Local::now();
        for score in candidates.values_mut() {
            if let Ok(timestamp) = DateTime::parse_from_rfc3339(&score.timestamp) {
                let timestamp_local = timestamp.with_timezone(&Local);
                let age_days = (now - timestamp_local).num_days().max(0) as f64;
                let recency = (1.0 - (age_days / MAX_DAYS).min(1.0)) * RECENCY_WEIGHT;
                score.recency_score = recency;
            }
        }

        // Sort by total score and get session metadata
        let mut scored_sessions: Vec<_> = candidates.into_iter().collect();
        scored_sessions.sort_by(|a, b| {
            let total_a = a.1.total_score();
            let total_b = b.1.total_score();
            total_b
                .partial_cmp(&total_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Fetch session metadata for top results
        let mut results = Vec::new();
        for (session_id, score) in scored_sessions.into_iter().take(limit) {
            let mut stmt = self
                .conn
                .prepare("SELECT * FROM sessions WHERE id = ?1")
                .map_err(|e| {
                    CldevError::Config(format!("Failed to prepare session query: {}", e))
                })?;

            if let Some(session) = stmt
                .query_row(params![session_id], |row| {
                    Self::row_to_session_metadata(row)
                })
                .optional()
                .map_err(|e| CldevError::Config(format!("Failed to fetch session: {}", e)))?
            {
                let matched_files = self.get_session_files(&session.id)?;
                let matched_tags = self.get_session_tags(&session.id)?;

                results.push(QueryResult {
                    session,
                    matched_files,
                    matched_tags,
                    relevance_score: score.total_score(),
                });
            }
        }

        Ok(results)
    }

    /// Calculate error similarity using simple token matching
    fn calculate_error_similarity(query: &str, stored: &str) -> f64 {
        let query_tokens: HashSet<&str> = query.split_whitespace().collect();
        let stored_tokens: HashSet<&str> = stored.split_whitespace().collect();

        if query_tokens.is_empty() || stored_tokens.is_empty() {
            return 0.0;
        }

        let intersection = query_tokens.intersection(&stored_tokens).count();
        let union = query_tokens.union(&stored_tokens).count();

        intersection as f64 / union as f64
    }

    /// Find sessions with similar error messages using Levenshtein distance
    ///
    /// Uses normalized similarity calculation to find sessions with errors similar to the query.
    /// Normalizes error messages by removing dynamic elements like paths, line numbers, hashes, etc.
    ///
    /// # Parameters
    ///
    /// - `error_query`: The error message to search for
    /// - `threshold`: Similarity threshold (0.0-1.0). Recommended: 0.7
    /// - `limit`: Maximum number of results
    ///
    /// # Returns
    ///
    /// Query results sorted by similarity score descending
    pub fn find_similar_errors(
        &self,
        error_query: &str,
        threshold: f64,
        limit: usize,
    ) -> Result<Vec<QueryResult>> {
        // Get all error patterns from the database
        let mut stmt = self
            .conn
            .prepare(
                r#"
            SELECT DISTINCT e.error_pattern, e.session_id
            FROM errors e
            "#,
            )
            .map_err(|e| CldevError::Config(format!("Failed to prepare query: {}", e)))?;

        let error_patterns = stmt
            .query_map(params![], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| CldevError::Config(format!("Failed to execute query: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| CldevError::Config(format!("Failed to collect errors: {}", e)))?;

        // Calculate similarity scores for each error pattern
        let scored_sessions: Vec<(String, f64)> = error_patterns
            .into_iter()
            .map(|(pattern, session_id)| {
                let similarity = calculate_similarity(error_query, &pattern);
                (session_id, similarity.score)
            })
            .filter(|(_, score)| *score >= threshold)
            .collect();

        // Deduplicate by session_id and keep highest score
        let mut session_scores: HashMap<String, f64> = HashMap::new();
        for (session_id, score) in scored_sessions {
            session_scores
                .entry(session_id)
                .and_modify(|existing_score| {
                    if score > *existing_score {
                        *existing_score = score;
                    }
                })
                .or_insert(score);
        }

        // Sort by score descending and limit results
        let mut results: Vec<(String, f64)> = session_scores.into_iter().collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        // Fetch session metadata for matched sessions
        let mut final_results = Vec::new();
        for (session_id, similarity_score) in results {
            let session_meta: SessionMetadata = self
                .conn
                .query_row(
                    "SELECT * FROM sessions WHERE id = ?1",
                    params![session_id],
                    Self::row_to_session_metadata,
                )
                .optional()
                .map_err(|e| CldevError::Config(format!("Failed to fetch session: {}", e)))?
                .ok_or_else(|| CldevError::Config(format!("Session not found: {}", session_id)))?;

            let matched_files = self.get_session_files(&session_id)?;
            let matched_tags = self.get_session_tags(&session_id)?;

            final_results.push(QueryResult {
                session: session_meta,
                matched_files,
                matched_tags,
                relevance_score: similarity_score,
            });
        }

        Ok(final_results)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_database_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");

        let result = LearningDatabase::new(db_path, markdown_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_stale_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let db = LearningDatabase::new(db_path, markdown_dir).unwrap();
        assert!(!db.is_stale().unwrap());
    }

    #[test]
    fn test_suggest_by_context_file_only() {
        use crate::core::learning_record_v2::*;

        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let mut db = LearningDatabase::new(db_path, markdown_dir.clone()).unwrap();

        // Create test record with file
        let record = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "File Test",
            "Testing file matching",
            Severity::Error,
        )
        .files(vec![FileAffected {
            path: "src/core/test.rs".to_string(),
            role: FileRole::Primary,
            changes_summary: None,
            hotspot_score: 1.0,
        }])
        .build();

        let path = markdown_dir.join(format!("{}.md", record.session_meta.id));
        let yaml = serde_yaml::to_string(&record).unwrap();
        let content = format!("---\n{}---\n", yaml);
        fs::write(&path, content).unwrap();

        db.upsert_session(&record, path.to_string_lossy().to_string())
            .unwrap();

        // Query by file
        let results = db
            .suggest_by_context(Some("test.rs"), None, None, 5)
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].session.title, "File Test");
        assert!(results[0].relevance_score >= 0.4); // At least file weight
    }

    #[test]
    fn test_suggest_by_context_composite() {
        use crate::core::learning_record_v2::*;

        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let mut db = LearningDatabase::new(db_path, markdown_dir.clone()).unwrap();

        // Create test record with multiple features
        let record = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "Composite Test",
            "Testing composite scoring",
            Severity::Error,
        )
        .files(vec![FileAffected {
            path: "src/main.rs".to_string(),
            role: FileRole::Primary,
            changes_summary: None,
            hotspot_score: 1.0,
        }])
        .tag("rust")
        .tag("performance")
        .build();

        let mut record_with_errors = record.clone();
        record_with_errors
            .problem
            .error_signatures
            .push(ErrorSignature {
                error_type: "RuntimeError".to_string(),
                pattern: "thread panicked at overflow".to_string(),
                stack_trace_hash: None,
            });

        let path = markdown_dir.join(format!("{}.md", record_with_errors.session_meta.id));
        let yaml = serde_yaml::to_string(&record_with_errors).unwrap();
        let content = format!("---\n{}---\n", yaml);
        fs::write(&path, content).unwrap();

        db.upsert_session(&record_with_errors, path.to_string_lossy().to_string())
            .unwrap();

        // Query with file + tags + error
        let results = db
            .suggest_by_context(
                Some("main.rs"),
                Some("overflow"),
                Some(&["rust".to_string()]),
                5,
            )
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].session.title, "Composite Test");
        // Composite: file(0.4) + tag(0.2) + error(~0.2) + recency(0.1) ≈ 0.9
        assert!(results[0].relevance_score >= 0.5);
    }

    #[test]
    fn test_error_similarity() {
        let query = "thread panicked at overflow";
        let stored1 = "thread panicked at overflow in main";
        let stored2 = "different error message";

        let sim1 = LearningDatabase::calculate_error_similarity(query, stored1);
        let sim2 = LearningDatabase::calculate_error_similarity(query, stored2);

        assert!(sim1 > 0.5); // High similarity
        assert!(sim2 < 0.3); // Low similarity
        assert!(sim1 > sim2); // sim1 should be higher
    }
}

/// Composite score for context-based suggestions
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CompositeScore {
    session_id: String,
    timestamp: String,
    file_score: f64,
    error_score: f64,
    tag_score: f64,
    recency_score: f64,
}

impl CompositeScore {
    fn total_score(&self) -> f64 {
        self.file_score + self.error_score + self.tag_score + self.recency_score
    }
}
