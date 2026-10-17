pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigsListDeclarationsRequest {
    /// List the systems of this country instead of the company country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl ConfigsListDeclarationsRequest {
    pub fn builder() -> ConfigsListDeclarationsRequestBuilder {
        <ConfigsListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsListDeclarationsRequestBuilder {
    country: Option<String>,
}

impl ConfigsListDeclarationsRequestBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfigsListDeclarationsRequest`].
    pub fn build(self) -> Result<ConfigsListDeclarationsRequest, BuildError> {
        Ok(ConfigsListDeclarationsRequest {
            country: self.country,
        })
    }
}
