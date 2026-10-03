pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAutomationUpdateResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsAutomationUpdateResponseRowsItem>,
}

impl PostV1DeclarationsAutomationUpdateResponse {
    pub fn builder() -> PostV1DeclarationsAutomationUpdateResponseBuilder {
        <PostV1DeclarationsAutomationUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAutomationUpdateResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsAutomationUpdateResponseRowsItem>>,
}

impl PostV1DeclarationsAutomationUpdateResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1DeclarationsAutomationUpdateResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAutomationUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsAutomationUpdateResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsAutomationUpdateResponse, BuildError> {
        Ok(PostV1DeclarationsAutomationUpdateResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
