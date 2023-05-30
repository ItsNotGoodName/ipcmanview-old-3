use super::{Page, PageResult};

impl Page {
    pub fn new(mut page: i32, mut per_page: i32) -> Self {
        if page < 1 {
            page = 1;
        }
        if per_page < 5 {
            per_page = 5;
        }
        if per_page > 100 {
            per_page = 100
        }

        Self { page, per_page }
    }

    pub fn offset(&self) -> i32 {
        (self.page - 1) * self.per_page
    }

    pub fn limit(&self) -> i32 {
        self.per_page
    }

    pub fn new_result<T>(&self, items: Vec<T>, total_items: i32) -> PageResult<T> {
        PageResult {
            page: self.page,
            per_page: self.per_page,
            total_pages: (total_items / self.per_page) + 1,
            total_items,
            items,
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 5,
        }
    }
}
