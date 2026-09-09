pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesOptionsResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LeadsSourcesOptionsResponseRowsItem>,
}

impl PostV1LeadsSourcesOptionsResponse {
    pub fn builder() -> PostV1LeadsSourcesOptionsResponseBuilder {
        <PostV1LeadsSourcesOptionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesOptionsResponseBuilder {
    rows: Option<Vec<PostV1LeadsSourcesOptionsResponseRowsItem>>,
}

impl PostV1LeadsSourcesOptionsResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LeadsSourcesOptionsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsSourcesOptionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LeadsSourcesOptionsResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1LeadsSourcesOptionsResponse, BuildError> {
        Ok(PostV1LeadsSourcesOptionsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
