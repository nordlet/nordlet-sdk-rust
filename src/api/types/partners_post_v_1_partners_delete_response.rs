pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersDeleteResponse {
    pub fn builder() -> PostV1PartnersDeleteResponseBuilder {
        <PostV1PartnersDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1PartnersDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersDeleteResponse, BuildError> {
        Ok(PostV1PartnersDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
