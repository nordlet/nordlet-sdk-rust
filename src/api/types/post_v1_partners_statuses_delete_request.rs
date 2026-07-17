pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersStatusesDeleteRequest {
    pub fn builder() -> PostV1PartnersStatusesDeleteRequestBuilder {
        <PostV1PartnersStatusesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersStatusesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersStatusesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersStatusesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersStatusesDeleteRequest, BuildError> {
        Ok(PostV1PartnersStatusesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
