//! Core functionality modules for cldev CLI
//!
//! This module contains the foundational components for configuration management,
//! security, and shared utilities.

#![allow(unused_imports)]

pub mod auto_capture;
pub mod config;
pub mod error;
pub mod git_utils;
pub mod i18n;
pub mod learning_db;
pub mod learning_index;
pub mod learning_record_v2;
pub mod learning_record_v3;
pub mod project_config;
pub mod project_detector;
pub mod sanitizer;
pub mod security;
pub mod session_context;
pub mod session_recorder;
pub mod similarity;
pub mod stack_config;
pub mod tfidf;

pub use auto_capture::{
    analyze_session, generate_level2_markdown, RecordLevel, RecordRecommendation,
};
pub use config::{Config, ConfigVersion, HierarchicalConfig};
pub use error::{CldevError, Result};
pub use git_utils::GitUtils;
pub use learning_db::{Hotspot, LearningDatabase, QueryResult, SessionMetadata};
pub use learning_index::{
    FileHotspot, LearningIndexV2, SessionRef, TimelineEntry, UnresolvedEntry,
};
pub use learning_record_v2::{
    Context, Dependency, ErrorSignature, FileAffected, FileRole, Learning, LearningRecordBuilder,
    LearningRecordV2, Priority, Problem, Reusability, SessionMeta, SessionType, Severity, Solution,
};
pub use learning_record_v3::{LearningRecordV3, RecordStatus};
pub use project_config::ProjectConfig;
pub use project_detector::{ProjectDetector, ProjectType};
pub use sanitizer::{sanitize_text, SanitizationResult};
pub use session_context::{
    CommandRecord, ErrorCapture, FileModification, ModificationType, SessionContext, TodoAction,
    TodoSnapshot, TodoStatus, ToolUsage,
};
pub use session_recorder::{LearningSession, LearningSessionBuilder};
pub use stack_config::{StackConfig, TechStack};
pub use tfidf::{IndexStats, TfidfIndex, TfidfResult};
