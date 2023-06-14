use ipcmanview::models::Page;

impl From<super::PageQuery> for Page {
    fn from(value: super::PageQuery) -> Self {
        Page::new(value.page, value.per_page)
    }
}
