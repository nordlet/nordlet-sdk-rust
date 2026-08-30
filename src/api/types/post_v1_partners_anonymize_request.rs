pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersAnonymizeRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersAnonymizeRequest {
    pub fn builder() -> PostV1PartnersAnonymizeRequestBuilder {
        <PostV1PartnersAnonymizeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAnonymizeRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersAnonymizeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersAnonymizeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersAnonymizeRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersAnonymizeRequest, BuildError> {
        Ok(PostV1PartnersAnonymizeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
