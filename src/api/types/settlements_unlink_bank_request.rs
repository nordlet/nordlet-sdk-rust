pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettlementsUnlinkBankRequest {
    #[serde(default)]
    pub id: String,
}

impl SettlementsUnlinkBankRequest {
    pub fn builder() -> SettlementsUnlinkBankRequestBuilder {
        <SettlementsUnlinkBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsUnlinkBankRequestBuilder {
    id: Option<String>,
}

impl SettlementsUnlinkBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SettlementsUnlinkBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SettlementsUnlinkBankRequestBuilder::id)
    pub fn build(self) -> Result<SettlementsUnlinkBankRequest, BuildError> {
        Ok(SettlementsUnlinkBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
