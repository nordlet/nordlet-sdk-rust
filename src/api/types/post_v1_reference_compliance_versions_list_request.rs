pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceComplianceVersionsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl PostV1ReferenceComplianceVersionsListRequest {
    pub fn builder() -> PostV1ReferenceComplianceVersionsListRequestBuilder {
        <PostV1ReferenceComplianceVersionsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceComplianceVersionsListRequestBuilder {
    country: Option<String>,
}

impl PostV1ReferenceComplianceVersionsListRequestBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceComplianceVersionsListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceComplianceVersionsListRequest, BuildError> {
        Ok(PostV1ReferenceComplianceVersionsListRequest {
            country: self.country,
        })
    }
}
