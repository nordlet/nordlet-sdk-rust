pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1TransportWaybillsIssueRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1TransportWaybillsIssueRequest {
    pub fn builder() -> PostV1TransportWaybillsIssueRequestBuilder {
        <PostV1TransportWaybillsIssueRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsIssueRequestBuilder {
    id: Option<String>,
}

impl PostV1TransportWaybillsIssueRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsIssueRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1TransportWaybillsIssueRequestBuilder::id)
    pub fn build(self) -> Result<PostV1TransportWaybillsIssueRequest, BuildError> {
        Ok(PostV1TransportWaybillsIssueRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
