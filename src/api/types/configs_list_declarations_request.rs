pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigsListDeclarationsRequest {}

impl ConfigsListDeclarationsRequest {
    pub fn builder() -> ConfigsListDeclarationsRequestBuilder {
        <ConfigsListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsListDeclarationsRequestBuilder {}

impl ConfigsListDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`ConfigsListDeclarationsRequest`].
    pub fn build(self) -> Result<ConfigsListDeclarationsRequest, BuildError> {
        Ok(ConfigsListDeclarationsRequest {})
    }
}
