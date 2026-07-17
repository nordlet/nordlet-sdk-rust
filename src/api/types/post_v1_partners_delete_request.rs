pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersDeleteRequest {
    pub fn builder() -> PostV1PartnersDeleteRequestBuilder {
        <PostV1PartnersDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersDeleteRequest, BuildError> {
        Ok(PostV1PartnersDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
