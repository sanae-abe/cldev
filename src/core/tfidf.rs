//! TF-IDF Search Engine for Learning Records
//!
//! Provides keyword-based ranking using Term Frequency-Inverse Document Frequency (TF-IDF).
//! This complements the existing SQLite FTS5 full-text search with more nuanced scoring.

use std::collections::HashMap;

/// TF-IDF search index
#[derive(Debug, Clone)]
pub struct TfidfIndex {
    /// Document count
    #[allow(dead_code)]
    doc_count: usize,
    /// Term document frequency: term -> number of documents containing the term
    #[allow(dead_code)]
    term_doc_freq: HashMap<String, usize>,
    /// Document term frequency: doc_id -> (term -> count)
    #[allow(dead_code)]
    doc_term_freq: HashMap<String, HashMap<String, usize>>,
    /// Document total word count: doc_id -> total words
    #[allow(dead_code)]
    doc_word_count: HashMap<String, usize>,
}

/// Search result with TF-IDF score
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TfidfResult {
    pub doc_id: String,
    pub score: f64,
}

impl TfidfIndex {
    /// Create a new empty TF-IDF index
    pub fn new() -> Self {
        Self {
            doc_count: 0,
            term_doc_freq: HashMap::new(),
            doc_term_freq: HashMap::new(),
            doc_word_count: HashMap::new(),
        }
    }

    /// Add a document to the index
    ///
    /// # Arguments
    /// * `doc_id` - Unique document identifier (session ID)
    /// * `text` - Document text to index
    pub fn add_document(&mut self, doc_id: impl Into<String>, text: &str) {
        let doc_id = doc_id.into();
        let terms = Self::tokenize(text);

        if terms.is_empty() {
            return;
        }

        // Count term frequencies in this document
        let mut term_freq = HashMap::new();
        let mut total_words = 0;

        for term in &terms {
            *term_freq.entry(term.clone()).or_insert(0) += 1;
            total_words += 1;
        }

        // Update document term frequency
        self.doc_term_freq.insert(doc_id.clone(), term_freq.clone());
        self.doc_word_count.insert(doc_id.clone(), total_words);

        // Update term document frequency
        for term in term_freq.keys() {
            *self.term_doc_freq.entry(term.clone()).or_insert(0) += 1;
        }

        self.doc_count += 1;
    }

    /// Remove a document from the index
    ///
    /// # Arguments
    /// * `doc_id` - Document identifier to remove
    pub fn remove_document(&mut self, doc_id: &str) -> bool {
        if let Some(term_freq) = self.doc_term_freq.remove(doc_id) {
            // Decrement term document frequencies
            for term in term_freq.keys() {
                if let Some(count) = self.term_doc_freq.get_mut(term) {
                    *count -= 1;
                    if *count == 0 {
                        self.term_doc_freq.remove(term);
                    }
                }
            }

            self.doc_word_count.remove(doc_id);
            self.doc_count -= 1;
            true
        } else {
            false
        }
    }

    /// Search documents by query with TF-IDF ranking
    ///
    /// # Arguments
    /// * `query` - Search query text
    /// * `limit` - Maximum number of results
    ///
    /// # Returns
    /// Vec of (doc_id, score) sorted by score descending
    #[allow(dead_code)]
    pub fn search(&self, query: &str, limit: usize) -> Vec<TfidfResult> {
        let query_terms = Self::tokenize(query);
        if query_terms.is_empty() {
            return Vec::new();
        }

        // Calculate TF-IDF scores for all documents
        let mut scores: Vec<(String, f64)> = self
            .doc_term_freq
            .keys()
            .map(|doc_id| {
                let score = self.calculate_document_score(doc_id, &query_terms);
                (doc_id.clone(), score)
            })
            .filter(|(_, score)| *score > 0.0)
            .collect();

        // Sort by score descending
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return top results
        scores
            .into_iter()
            .take(limit)
            .map(|(doc_id, score)| TfidfResult { doc_id, score })
            .collect()
    }

    /// Calculate TF-IDF score for a document given query terms
    #[allow(dead_code)]
    fn calculate_document_score(&self, doc_id: &str, query_terms: &[String]) -> f64 {
        let doc_term_freq = match self.doc_term_freq.get(doc_id) {
            Some(freq) => freq,
            None => return 0.0,
        };

        let doc_word_count = self.doc_word_count.get(doc_id).copied().unwrap_or(1);

        let mut score = 0.0;

        for term in query_terms {
            if let Some(&term_count) = doc_term_freq.get(term) {
                let tf = self.calculate_tf(term_count, doc_word_count);
                let idf = self.calculate_idf(term);
                score += tf * idf;
            }
        }

        score
    }

    /// Calculate Term Frequency (TF)
    ///
    /// TF = (term_count) / (total_words_in_document)
    #[allow(dead_code)]
    fn calculate_tf(&self, term_count: usize, doc_word_count: usize) -> f64 {
        term_count as f64 / doc_word_count as f64
    }

    /// Calculate Inverse Document Frequency (IDF)
    ///
    /// IDF = log(total_documents / documents_containing_term)
    #[allow(dead_code)]
    fn calculate_idf(&self, term: &str) -> f64 {
        let doc_freq = self.term_doc_freq.get(term).copied().unwrap_or(0);
        if doc_freq == 0 {
            return 0.0;
        }

        let total_docs = self.doc_count as f64;
        let docs_with_term = doc_freq as f64;

        (total_docs / docs_with_term).ln()
    }

    /// Tokenize text into lowercase terms
    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
            .filter(|s| !s.is_empty())
            .filter(|s| s.len() >= 2) // Filter out single-character terms
            .map(|s| s.to_string())
            .collect()
    }

    /// Get index statistics
    #[allow(dead_code)]
    pub fn stats(&self) -> IndexStats {
        IndexStats {
            doc_count: self.doc_count,
            term_count: self.term_doc_freq.len(),
            avg_doc_length: if self.doc_count > 0 {
                self.doc_word_count.values().sum::<usize>() as f64 / self.doc_count as f64
            } else {
                0.0
            },
        }
    }
}

/// Index statistics
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IndexStats {
    pub doc_count: usize,
    pub term_count: usize,
    pub avg_doc_length: f64,
}

impl Default for TfidfIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = TfidfIndex::tokenize("Hello, World! This is a test.");
        assert_eq!(tokens, vec!["hello", "world", "this", "is", "test"]);
    }

    #[test]
    fn test_tokenize_filters_short() {
        let tokens = TfidfIndex::tokenize("a b cd ef ghi");
        assert_eq!(tokens, vec!["cd", "ef", "ghi"]);
    }

    #[test]
    fn test_add_document() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust programming language");

        assert_eq!(index.doc_count, 1);
        assert_eq!(index.term_doc_freq.len(), 3);
        assert!(index.doc_term_freq.contains_key("doc1"));
    }

    #[test]
    fn test_remove_document() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust programming");
        index.add_document("doc2", "python programming");

        assert_eq!(index.doc_count, 2);

        let removed = index.remove_document("doc1");
        assert!(removed);
        assert_eq!(index.doc_count, 1);
        assert!(!index.doc_term_freq.contains_key("doc1"));
    }

    #[test]
    fn test_calculate_tf() {
        let index = TfidfIndex::new();
        let tf = index.calculate_tf(3, 10);
        assert_eq!(tf, 0.3);
    }

    #[test]
    fn test_calculate_idf() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust programming");
        index.add_document("doc2", "python programming");
        index.add_document("doc3", "java coding");

        // "programming" appears in 2 out of 3 documents
        let idf_programming = index.calculate_idf("programming");
        let expected_idf = (3.0_f64 / 2.0_f64).ln();
        assert!((idf_programming - expected_idf).abs() < 0.001);

        // "rust" appears in 1 out of 3 documents
        let idf_rust = index.calculate_idf("rust");
        let expected_idf_rust = (3.0_f64 / 1.0_f64).ln();
        assert!((idf_rust - expected_idf_rust).abs() < 0.001);
    }

    #[test]
    fn test_search() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust programming language for systems");
        index.add_document("doc2", "python programming language for data science");
        index.add_document("doc3", "java programming language");

        let results = index.search("rust systems", 10);

        // doc1 should rank highest
        assert!(!results.is_empty());
        assert_eq!(results[0].doc_id, "doc1");
    }

    #[test]
    fn test_search_ranking() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust rust rust programming");
        index.add_document("doc2", "rust programming language");
        index.add_document("doc3", "python programming");

        let results = index.search("rust", 10);

        // doc1 has highest TF for "rust" (3 occurrences)
        assert_eq!(results[0].doc_id, "doc1");
        assert!(results[0].score > results[1].score);
    }

    #[test]
    fn test_search_empty_query() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust programming");

        let results = index.search("", 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_stats() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust programming language");
        index.add_document("doc2", "python coding");

        let stats = index.stats();
        assert_eq!(stats.doc_count, 2);
        assert_eq!(stats.term_count, 5); // rust, programming, language, python, coding
        assert!(stats.avg_doc_length > 0.0);
    }

    #[test]
    fn test_search_no_match() {
        let mut index = TfidfIndex::new();
        index.add_document("doc1", "rust programming");

        let results = index.search("javascript", 10);
        assert!(results.is_empty());
    }
}
