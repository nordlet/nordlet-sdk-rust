pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersStatusesDeleteResponse {
    pub fn builder() -> PostV1PartnersStatusesDeleteResponseBuilder {
        <PostV1PartnersStatusesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1PartnersStatusesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersStatusesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersStatusesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersStatusesDeleteResponse, BuildError> {
        Ok(PostV1PartnersStatusesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
