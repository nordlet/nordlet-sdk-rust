pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1AssetsAssetsGetRequest {
    pub fn builder() -> PostV1AssetsAssetsGetRequestBuilder {
        <PostV1AssetsAssetsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1AssetsAssetsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AssetsAssetsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AssetsAssetsGetRequest, BuildError> {
        Ok(PostV1AssetsAssetsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
