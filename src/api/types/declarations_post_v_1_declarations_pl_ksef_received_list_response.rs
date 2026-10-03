pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlKsefReceivedListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsPlKsefReceivedListResponseRowsItem>,
}

impl PostV1DeclarationsPlKsefReceivedListResponse {
    pub fn builder() -> PostV1DeclarationsPlKsefReceivedListResponseBuilder {
        <PostV1DeclarationsPlKsefReceivedListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlKsefReceivedListResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsPlKsefReceivedListResponseRowsItem>>,
}

impl PostV1DeclarationsPlKsefReceivedListResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsPlKsefReceivedListResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlKsefReceivedListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsPlKsefReceivedListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsPlKsefReceivedListResponse, BuildError> {
        Ok(PostV1DeclarationsPlKsefReceivedListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
