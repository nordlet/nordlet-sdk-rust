pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersGetRequest {
    pub fn builder() -> PostV1PartnersGetRequestBuilder {
        <PostV1PartnersGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersGetRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersGetRequest, BuildError> {
        Ok(PostV1PartnersGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
