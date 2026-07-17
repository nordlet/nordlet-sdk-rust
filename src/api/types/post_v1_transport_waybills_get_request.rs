pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1TransportWaybillsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1TransportWaybillsGetRequest {
    pub fn builder() -> PostV1TransportWaybillsGetRequestBuilder {
        <PostV1TransportWaybillsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1TransportWaybillsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1TransportWaybillsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1TransportWaybillsGetRequest, BuildError> {
        Ok(PostV1TransportWaybillsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
