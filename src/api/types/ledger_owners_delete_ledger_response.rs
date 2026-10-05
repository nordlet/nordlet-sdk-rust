pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnersDeleteLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub deleted: bool,
}

impl OwnersDeleteLedgerResponse {
    pub fn builder() -> OwnersDeleteLedgerResponseBuilder {
        <OwnersDeleteLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnersDeleteLedgerResponseBuilder {
    id: Option<String>,
    deleted: Option<bool>,
}

impl OwnersDeleteLedgerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnersDeleteLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OwnersDeleteLedgerResponseBuilder::id)
    /// - [`deleted`](OwnersDeleteLedgerResponseBuilder::deleted)
    pub fn build(self) -> Result<OwnersDeleteLedgerResponse, BuildError> {
        Ok(OwnersDeleteLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
