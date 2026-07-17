pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsIssueRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesActsIssueRequest {
    pub fn builder() -> PostV1SalesActsIssueRequestBuilder {
        <PostV1SalesActsIssueRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsIssueRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesActsIssueRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsIssueRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesActsIssueRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesActsIssueRequest, BuildError> {
        Ok(PostV1SalesActsIssueRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
