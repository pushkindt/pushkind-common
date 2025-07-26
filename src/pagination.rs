//! Helpers for building page navigation widgets.
//!
//! The pagination algorithm computes a list of page numbers with optional
//! ellipses so templates can render compact navigation controls. A few pages
//! from the beginning and end are always shown while the pages around the
//! current one remain visible as well.

use serde::Serialize;

/// Default number of list items shown when a page size is not specified.
/// This constant is used by pagination helpers throughout the crate.
pub const DEFAULT_ITEMS_PER_PAGE: usize = 20;

/// Calculate a list of page numbers for a pagination bar.
///
/// `total_pages` is the total number of pages. `current_page` denotes the
/// page currently displayed. `left_edge` and `right_edge` specify how many
/// pages are always shown at the start and end of the pagination sequence.
/// `left_current` and `right_current` control how many pages are visible on
/// each side of the current page. `None` values in the resulting vector
/// represent collapsed ranges (ellipses).
fn get_pages(
    total_pages: usize,
    current_page: usize,
    left_edge: usize,
    left_current: usize,
    right_current: usize,
    right_edge: usize,
) -> Vec<Option<usize>> {
    let last_page = total_pages;

    if last_page == 0 {
        return vec![];
    }

    let mut pages = Vec::new();

    let left_end = (1 + left_edge).min(last_page + 1);
    pages.extend((1..left_end).map(Some));

    let mid_start = left_end.max(current_page.saturating_sub(left_current));
    let mid_end = (current_page + right_current + 1).min(last_page + 1);

    if mid_start > left_end {
        pages.push(None);
    }
    pages.extend((mid_start..mid_end).map(Some));

    let right_start = mid_end.max(last_page.saturating_sub(right_edge) + 1);

    if right_start > mid_end {
        pages.push(None);
    }
    pages.extend((right_start..=last_page).map(Some));

    pages
}

#[derive(Serialize)]
/// Items of a single page together with pagination info.
///
/// The sequence of page numbers is stored in `pages` and is suitable for
/// building navigation controls. `page` keeps the normalized current page
/// number.
pub struct Paginated<T> {
    items: Vec<T>,
    pages: Vec<Option<usize>>,
    page: usize,
}

#[derive(Debug, Clone)]
/// A helper struct to provide pagination information
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: DEFAULT_ITEMS_PER_PAGE,
        }
    }
}

impl<T> Paginated<T> {
    /// Create a [`Paginated`] value from a list of items.
    ///
    /// A `current_page` of zero is interpreted as page one. The page list is
    /// generated using [`get_pages`].
    pub fn new(items: Vec<T>, current_page: usize, total_pages: usize) -> Self {
        let current_page = if current_page == 0 { 1 } else { current_page };

        let pages = get_pages(total_pages, current_page, 2, 2, 4, 2);

        Self {
            items,
            pages,
            page: current_page,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_without_ellipses() {
        let pages = get_pages(10, 5, 2, 2, 4, 2);
        let expected = (1..=10).map(Some).collect::<Vec<_>>();
        assert_eq!(pages, expected);
    }

    #[test]
    fn pages_with_ellipses() {
        let pages = get_pages(100, 1, 2, 2, 4, 2);
        assert_eq!(
            pages,
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                None,
                Some(99),
                Some(100),
            ]
        );
    }

    #[test]
    fn paginated_sets_page_to_one_when_zero() {
        let paginated: Paginated<i32> = Paginated::new(vec![1, 2, 3], 0, 3);
        assert_eq!(paginated.page, 1);
        assert_eq!(paginated.pages, vec![Some(1), Some(2), Some(3)]);
    }

    #[test]
    fn pages_empty_when_no_pages() {
        let pages = get_pages(0, 1, 2, 2, 4, 2);
        assert!(pages.is_empty());
    }
}
