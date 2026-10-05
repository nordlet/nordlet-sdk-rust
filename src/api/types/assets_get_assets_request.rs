pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsGetAssetsRequest {
    #[serde(default)]
    pub id: String,
}

impl AssetsGetAssetsRequest {
    pub fn builder() -> AssetsGetAssetsRequestBuilder {
        <AssetsGetAssetsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsGetAssetsRequestBuilder {
    id: Option<String>,
}

impl AssetsGetAssetsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsGetAssetsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssetsGetAssetsRequestBuilder::id)
    pub fn build(self) -> Result<AssetsGetAssetsRequest, BuildError> {
        Ok(AssetsGetAssetsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
