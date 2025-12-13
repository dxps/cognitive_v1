use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

impl Pagination {
    /// Get the pagination's `offset` and `limit` options.
    pub fn get_offset_limit(&self) -> (i32, i32) {
        let limit = self.limit.unwrap_or(10);
        let offset = (self.page.unwrap_or(1) - 1) * limit;
        (offset, limit)
    }

    /// Instantiate a pagination from optional values.
    /// If no values are provided, the default values are page=1 and limit=10.
    pub fn from(pagination_opt: Option<&Pagination>) -> Self {
        Self {
            page: pagination_opt.map(|p| p.page.unwrap_or(1)),
            limit: pagination_opt.map(|p| p.limit.unwrap_or(10)),
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(10),
        }
    }
}
