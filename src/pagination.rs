use serde::Deserialize;

use crate::models::PaginationInfo;

/// Parameters for paginated requests.
#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub page: u32,
    pub per_page: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 50,
        }
    }
}

impl PaginationParams {
    pub fn new(page: u32, per_page: u32) -> Self {
        Self { page, per_page }
    }

    pub(crate) fn as_query_pairs(&self) -> Vec<(&str, String)> {
        vec![
            ("page", self.page.to_string()),
            ("per_page", self.per_page.to_string()),
        ]
    }
}

/// A paginated response containing items and pagination metadata.
#[derive(Debug)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub pagination: PaginationInfo,
    /// Closure-like state for fetching the next page. We store enough
    /// context for the caller to request the next page themselves.
    next_page_num: Option<u32>,
    per_page: u32,
}

impl<T> Paginated<T> {
    pub(crate) fn new(items: Vec<T>, pagination: PaginationInfo) -> Self {
        let next_page_num = if pagination.page < pagination.pages {
            Some(pagination.page + 1)
        } else {
            None
        };
        let per_page = pagination.per_page;
        Self {
            items,
            pagination,
            next_page_num,
            per_page,
        }
    }

    /// Returns true if there are more pages available.
    pub fn has_next(&self) -> bool {
        self.next_page_num.is_some()
    }

    /// Returns the pagination params for the next page, if any.
    pub fn next_page_params(&self) -> Option<PaginationParams> {
        self.next_page_num.map(|page| PaginationParams {
            page,
            per_page: self.per_page,
        })
    }

    /// Returns the total number of items across all pages.
    pub fn total_items(&self) -> u32 {
        self.pagination.items
    }
}

/// Helper for deserializing paginated responses with varying JSON keys.
#[derive(Debug, Deserialize)]
pub(crate) struct PaginatedResponse<T> {
    pub pagination: PaginationInfo,
    #[serde(flatten)]
    pub data: PaginatedData<T>,
}

/// Captures the varying key names used by different endpoints.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum PaginatedData<T> {
    Releases { releases: Vec<T> },
    Versions { versions: Vec<T> },
    Results { results: Vec<T> },
}

impl<T> PaginatedData<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            PaginatedData::Releases { releases } => releases,
            PaginatedData::Versions { versions } => versions,
            PaginatedData::Results { results } => results,
        }
    }
}
