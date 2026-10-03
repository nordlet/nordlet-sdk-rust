pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAutomationListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsAutomationListResponseRowsItem>,
}

impl PostV1DeclarationsAutomationListResponse {
    pub fn builder() -> PostV1DeclarationsAutomationListResponseBuilder {
        <PostV1DeclarationsAutomationListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAutomationListResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsAutomationListResponseRowsItem>>,
}

impl PostV1DeclarationsAutomationListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1DeclarationsAutomationListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAutomationListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsAutomationListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsAutomationListResponse, BuildError> {
        Ok(PostV1DeclarationsAutomationListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
