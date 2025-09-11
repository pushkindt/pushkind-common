//! Helpers for implementing data repositories.
//!
//! This module collects error types and other utilities used by repository
//! implementations throughout the project.

pub mod errors;

/// Prepares user input for SQLite FTS5 MATCH by:
/// - replacing non-alphanumeric chars with spaces
/// - collapsing multiple spaces
/// - appending `*` for prefix search if not already present
pub fn build_fts_match_query(raw: &str) -> Option<String> {
    let mut result = String::with_capacity(raw.len());

    // Replace punctuation with spaces
    let mut prev_space = true;
    for ch in raw.chars() {
        if ch.is_alphanumeric() {
            result.push(ch);
            prev_space = false;
        } else if !prev_space {
            result.push(' ');
            prev_space = true;
        }
    }

    // Trim any trailing space introduced by punctuation at the end
    while result.ends_with(' ') {
        result.pop();
    }

    if result.is_empty() {
        return None;
    }

    // Add * to the last token if user didn't explicitly put one
    if !result.ends_with('*') {
        result.push('*');
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::build_fts_match_query;

    #[test]
    fn empty_input_returns_none() {
        assert_eq!(build_fts_match_query(""), None);
        assert_eq!(build_fts_match_query("   \t  \n"), None);
        assert_eq!(build_fts_match_query("..."), None);
    }

    #[test]
    fn dot_separated_tokens_are_split_and_starred() {
        assert_eq!(
            build_fts_match_query("gmail.com"),
            Some("gmail com*".into())
        );
        assert_eq!(
            build_fts_match_query("john.doe@example.com"),
            Some("john doe example com*".into())
        );
    }

    #[test]
    fn trailing_punctuation_is_ignored_and_star_added() {
        assert_eq!(build_fts_match_query("john."), Some("john*".into()));
        assert_eq!(build_fts_match_query("john-"), Some("john*".into()));
    }

    #[test]
    fn whitespace_is_collapsed_and_last_token_starred() {
        assert_eq!(
            build_fts_match_query("  john   doe  "),
            Some("john doe*".into())
        );
    }

    #[test]
    fn keeps_existing_star_on_last_token() {
        assert_eq!(build_fts_match_query("john*"), Some("john*".into()));
        assert_eq!(build_fts_match_query("john doe*"), Some("john doe*".into()));
    }

    #[test]
    fn unicode_is_supported() {
        assert_eq!(build_fts_match_query("Иванов.И."), Some("Иванов И*".into()));
    }
}
