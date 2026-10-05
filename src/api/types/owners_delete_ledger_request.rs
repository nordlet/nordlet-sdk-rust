pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnersDeleteLedgerRequest {
    #[serde(default)]
    pub id: String,
}

impl OwnersDeleteLedgerRequest {
    pub fn builder() -> OwnersDeleteLedgerRequestBuilder {
        <OwnersDeleteLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnersDeleteLedgerRequestBuilder {
    id: Option<String>,
}

impl OwnersDeleteLedgerRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OwnersDeleteLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OwnersDeleteLedgerRequestBuilder::id)
    pub fn build(self) -> Result<OwnersDeleteLedgerRequest, BuildError> {
        Ok(OwnersDeleteLedgerRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
