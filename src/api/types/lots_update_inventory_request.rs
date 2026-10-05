pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LotsUpdateInventoryRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl LotsUpdateInventoryRequest {
    pub fn builder() -> LotsUpdateInventoryRequestBuilder {
        <LotsUpdateInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LotsUpdateInventoryRequestBuilder {
    id: Option<String>,
    expiry_date: Option<NaiveDate>,
    notes: Option<String>,
}

impl LotsUpdateInventoryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn expiry_date(mut self, value: NaiveDate) -> Self {
        self.expiry_date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LotsUpdateInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LotsUpdateInventoryRequestBuilder::id)
    pub fn build(self) -> Result<LotsUpdateInventoryRequest, BuildError> {
        Ok(LotsUpdateInventoryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            expiry_date: self.expiry_date,
            notes: self.notes,
        })
    }
}
