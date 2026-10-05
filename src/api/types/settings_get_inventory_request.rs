pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsGetInventoryRequest {}

impl SettingsGetInventoryRequest {
    pub fn builder() -> SettingsGetInventoryRequestBuilder {
        <SettingsGetInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetInventoryRequestBuilder {}

impl SettingsGetInventoryRequestBuilder {
    /// Consumes the builder and constructs a [`SettingsGetInventoryRequest`].
    pub fn build(self) -> Result<SettingsGetInventoryRequest, BuildError> {
        Ok(SettingsGetInventoryRequest {})
    }
}
