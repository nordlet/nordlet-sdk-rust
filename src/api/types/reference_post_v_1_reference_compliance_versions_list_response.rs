pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceComplianceVersionsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceComplianceVersionsListResponseRowsItem>,
}

impl PostV1ReferenceComplianceVersionsListResponse {
    pub fn builder() -> PostV1ReferenceComplianceVersionsListResponseBuilder {
        <PostV1ReferenceComplianceVersionsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceComplianceVersionsListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceComplianceVersionsListResponseRowsItem>>,
}

impl PostV1ReferenceComplianceVersionsListResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1ReferenceComplianceVersionsListResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceComplianceVersionsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceComplianceVersionsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceComplianceVersionsListResponse, BuildError> {
        Ok(PostV1ReferenceComplianceVersionsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
