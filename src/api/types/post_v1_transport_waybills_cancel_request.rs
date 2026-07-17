pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1TransportWaybillsCancelRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1TransportWaybillsCancelRequest {
    pub fn builder() -> PostV1TransportWaybillsCancelRequestBuilder {
        <PostV1TransportWaybillsCancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsCancelRequestBuilder {
    id: Option<String>,
}

impl PostV1TransportWaybillsCancelRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsCancelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1TransportWaybillsCancelRequestBuilder::id)
    pub fn build(self) -> Result<PostV1TransportWaybillsCancelRequest, BuildError> {
        Ok(PostV1TransportWaybillsCancelRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
