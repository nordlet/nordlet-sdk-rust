pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CalendarListResponseRowsItem>,
}

impl PostV1CalendarListResponse {
    pub fn builder() -> PostV1CalendarListResponseBuilder {
        <PostV1CalendarListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarListResponseBuilder {
    rows: Option<Vec<PostV1CalendarListResponseRowsItem>>,
}

impl PostV1CalendarListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CalendarListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CalendarListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CalendarListResponse, BuildError> {
        Ok(PostV1CalendarListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
