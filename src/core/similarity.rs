//! Error message similarity calculation
//!
//! Provides fuzzy matching for error messages to identify similar problems across sessions.
//! Uses Levenshtein distance and normalized comparison to handle variations in:
//! - File paths
//! - Line numbers
//! - Hash values
//! - Timestamps
//! - Memory addresses

use strsim::levenshtein;

/// Similarity score result
#[derive(Debug, Clone)]
pub struct SimilarityScore {
    pub score: f64,
    #[allow(dead_code)]
    pub normalized_query: String,
    #[allow(dead_code)]
    pub normalized_target: String,
}

/// Normalize an error message by removing dynamic elements
///
/// This function removes:
/// - File paths (e.g., `/path/to/file.rs:123`)
/// - Line and column numbers (e.g., `:123:45`)
/// - Hash values (e.g., `abc123def456`)
/// - Memory addresses (e.g., `0x7fff12345678`)
/// - Timestamps (e.g., `2024-01-15 10:30:45`)
/// - Numeric IDs and version numbers
///
/// # Examples
///
/// ```
/// use cldev::core::similarity::normalize_error_message;
///
/// let error = "Error at src/main.rs:42: Undefined variable 'foo'";
/// let normalized = normalize_error_message(error);
/// assert_eq!(normalized, "error at undefined variable");
/// ```
pub fn normalize_error_message(error: &str) -> String {
    let mut normalized = error.to_lowercase();

    // Remove timestamps FIRST (before other patterns that might match parts of timestamps)
    // Note: [Tt\s] to handle both uppercase and lowercase after lowercasing
    normalized = regex::Regex::new(r"\[\d{4}-\d{2}-\d{2}[Tt\s]\d{2}:\d{2}:\d{2}\]")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();
    normalized = regex::Regex::new(r"\d{4}-\d{2}-\d{2}[Tt\s]\d{2}:\d{2}:\d{2}")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();
    normalized = regex::Regex::new(r"\d{2}:\d{2}:\d{2}")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();

    // Remove file paths with line numbers (e.g., src/main.rs:42 or /path/to/file:123:45)
    normalized = regex::Regex::new(r"[a-zA-Z0-9_/\.\-]+\.[a-z]+:\d+(:\d+)?")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();

    // Remove standalone line/column references (e.g., :123, :123:45)
    normalized = regex::Regex::new(r":\d+(:\d+)?")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();

    // Remove hex hashes and memory addresses (e.g., 0x7fff12345678, abc123def)
    normalized = regex::Regex::new(r"\b0x[a-f0-9]+\b")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();
    normalized = regex::Regex::new(r"\b[a-f0-9]{8,}\b")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();

    // Remove numeric IDs and version numbers (e.g., v1.2.3, id:12345)
    normalized = regex::Regex::new(r"\bv?\d+\.\d+(\.\d+)?\b")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();
    normalized = regex::Regex::new(r"\bid:\s*\d+\b")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();

    // Remove standalone numbers longer than 2 digits
    normalized = regex::Regex::new(r"\b\d{3,}\b")
        .unwrap()
        .replace_all(&normalized, "")
        .to_string();

    // Normalize whitespace and punctuation
    normalized = regex::Regex::new(r"[^\w\s]")
        .unwrap()
        .replace_all(&normalized, " ")
        .to_string();
    normalized = regex::Regex::new(r"\s+")
        .unwrap()
        .replace_all(&normalized, " ")
        .to_string();

    normalized.trim().to_string()
}

/// Calculate similarity score between two error messages
///
/// Uses normalized Levenshtein distance to compute a similarity score between 0.0 and 1.0.
/// - 1.0: Identical messages
/// - 0.7+: Very similar (recommended threshold)
/// - 0.5-0.7: Somewhat similar
/// - <0.5: Different messages
///
/// # Algorithm
///
/// 1. Normalize both error messages
/// 2. Calculate Levenshtein distance
/// 3. Normalize by maximum length: `score = 1.0 - (distance / max_length)`
///
/// # Examples
///
/// ```
/// use cldev::core::similarity::calculate_similarity;
///
/// let error1 = "Error at src/main.rs:42: Undefined variable 'foo'";
/// let error2 = "Error at src/utils.rs:123: Undefined variable 'bar'";
/// let score = calculate_similarity(error1, error2);
/// assert!(score.score > 0.7); // Very similar despite different paths
/// ```
pub fn calculate_similarity(error1: &str, error2: &str) -> SimilarityScore {
    let normalized1 = normalize_error_message(error1);
    let normalized2 = normalize_error_message(error2);

    // Handle empty strings
    if normalized1.is_empty() && normalized2.is_empty() {
        return SimilarityScore {
            score: 1.0,
            normalized_query: normalized1,
            normalized_target: normalized2,
        };
    }

    if normalized1.is_empty() || normalized2.is_empty() {
        return SimilarityScore {
            score: 0.0,
            normalized_query: normalized1,
            normalized_target: normalized2,
        };
    }

    // Calculate Levenshtein distance
    let distance = levenshtein(&normalized1, &normalized2);
    let max_length = normalized1.len().max(normalized2.len()) as f64;

    // Normalize to 0.0-1.0 range
    let score = 1.0 - (distance as f64 / max_length);

    SimilarityScore {
        score,
        normalized_query: normalized1,
        normalized_target: normalized2,
    }
}

/// Find similar strings in a list
///
/// Returns all strings with similarity score >= threshold, sorted by score descending.
///
/// # Examples
///
/// ```
/// use cldev::core::similarity::find_similar;
///
/// let query = "Error: Undefined variable 'foo'";
/// let candidates = vec![
///     "Error: Undefined variable 'bar'",
///     "Warning: Unused variable",
///     "Error: Type mismatch",
/// ];
/// let similar = find_similar(query, &candidates, 0.7);
/// assert_eq!(similar.len(), 1); // Only first candidate is similar enough
/// ```
#[allow(dead_code)]
pub fn find_similar(query: &str, candidates: &[&str], threshold: f64) -> Vec<(String, f64)> {
    let mut results: Vec<(String, f64)> = candidates
        .iter()
        .map(|&candidate| {
            let score = calculate_similarity(query, candidate);
            (candidate.to_string(), score.score)
        })
        .filter(|(_, score)| *score >= threshold)
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_basic() {
        let error = "Error at src/main.rs:42: Undefined variable";
        let normalized = normalize_error_message(error);
        assert_eq!(normalized, "error at undefined variable");
    }

    #[test]
    fn test_normalize_paths() {
        let error = "Error in /path/to/file.rs:123:45";
        let normalized = normalize_error_message(error);
        assert_eq!(normalized, "error in");
    }

    #[test]
    fn test_normalize_hex_addresses() {
        let error = "Segfault at 0x7fff12345678 hash abc123def456";
        let normalized = normalize_error_message(error);
        assert_eq!(normalized, "segfault at hash");
    }

    #[test]
    fn test_normalize_timestamps() {
        let error = "[2024-01-15T10:30:45] Error occurred";
        let normalized = normalize_error_message(error);
        assert_eq!(normalized, "error occurred");
    }

    #[test]
    fn test_normalize_version_numbers() {
        let error = "Version v1.2.3 incompatible with v2.0.0";
        let normalized = normalize_error_message(error);
        assert_eq!(normalized, "version incompatible with");
    }

    #[test]
    fn test_calculate_similarity_identical() {
        let error1 = "Undefined variable 'foo'";
        let error2 = "Undefined variable 'foo'";
        let score = calculate_similarity(error1, error2);
        assert_eq!(score.score, 1.0);
    }

    #[test]
    fn test_calculate_similarity_similar_with_paths() {
        let error1 = "Error at src/main.rs:42: Undefined variable 'foo'";
        let error2 = "Error at src/utils.rs:123: Undefined variable 'bar'";
        let score = calculate_similarity(error1, error2);
        assert!(score.score > 0.7, "Score: {}", score.score);
    }

    #[test]
    fn test_calculate_similarity_different() {
        let error1 = "Undefined variable";
        let error2 = "Type mismatch";
        let score = calculate_similarity(error1, error2);
        assert!(score.score < 0.5, "Score: {}", score.score);
    }

    #[test]
    fn test_calculate_similarity_empty() {
        let score1 = calculate_similarity("", "");
        assert_eq!(score1.score, 1.0);

        let score2 = calculate_similarity("test", "");
        assert_eq!(score2.score, 0.0);
    }

    #[test]
    fn test_find_similar() {
        let query = "Error: Undefined variable 'foo'";
        let candidates = vec![
            "Error: Undefined variable 'bar'",
            "Warning: Unused variable",
            "Error: Type mismatch",
        ];

        let similar = find_similar(query, &candidates, 0.7);
        assert_eq!(similar.len(), 1);
        assert!(similar[0].0.contains("Undefined variable"));
    }

    #[test]
    fn test_find_similar_multiple() {
        let query = "Cannot find module";
        let candidates = vec![
            "Cannot find module 'fs'",
            "Cannot find module 'path'",
            "Module not found",
            "Type error",
        ];

        let similar = find_similar(query, &candidates, 0.6);
        assert!(similar.len() >= 2);
        assert!(similar[0].1 >= 0.6);
    }

    #[test]
    fn test_find_similar_sorted() {
        let query = "Error occurred";
        let candidates = vec![
            "Error occurred at line 42",
            "Warning occurred",
            "Error happened",
        ];

        let similar = find_similar(query, &candidates, 0.5);
        assert!(!similar.is_empty());
        // First result should have highest score
        for i in 1..similar.len() {
            assert!(similar[i - 1].1 >= similar[i].1);
        }
    }
}
