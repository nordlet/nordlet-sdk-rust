pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsConfigsUpdateRequest {
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub config: HashMap<String, String>,
}

impl PostV1DeclarationsConfigsUpdateRequest {
    pub fn builder() -> PostV1DeclarationsConfigsUpdateRequestBuilder {
        <PostV1DeclarationsConfigsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsUpdateRequestBuilder {
    system: Option<String>,
    config: Option<HashMap<String, String>>,
}

impl PostV1DeclarationsConfigsUpdateRequestBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn config(mut self, value: HashMap<String, String>) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](PostV1DeclarationsConfigsUpdateRequestBuilder::system)
    /// - [`config`](PostV1DeclarationsConfigsUpdateRequestBuilder::config)
    pub fn build(self) -> Result<PostV1DeclarationsConfigsUpdateRequest, BuildError> {
        Ok(PostV1DeclarationsConfigsUpdateRequest {
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
