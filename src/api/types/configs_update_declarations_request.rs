pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConfigsUpdateDeclarationsRequest {
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub config: HashMap<String, String>,
}

impl ConfigsUpdateDeclarationsRequest {
    pub fn builder() -> ConfigsUpdateDeclarationsRequestBuilder {
        <ConfigsUpdateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsUpdateDeclarationsRequestBuilder {
    system: Option<String>,
    config: Option<HashMap<String, String>>,
}

impl ConfigsUpdateDeclarationsRequestBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn config(mut self, value: HashMap<String, String>) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigsUpdateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](ConfigsUpdateDeclarationsRequestBuilder::system)
    /// - [`config`](ConfigsUpdateDeclarationsRequestBuilder::config)
    pub fn build(self) -> Result<ConfigsUpdateDeclarationsRequest, BuildError> {
        Ok(ConfigsUpdateDeclarationsRequest {
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
