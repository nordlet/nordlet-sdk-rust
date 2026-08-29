pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLotsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1InventoryLotsUpdateRequest {
    pub fn builder() -> PostV1InventoryLotsUpdateRequestBuilder {
        <PostV1InventoryLotsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLotsUpdateRequestBuilder {
    id: Option<String>,
    expiry_date: Option<String>,
    notes: Option<String>,
}

impl PostV1InventoryLotsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn expiry_date(mut self, value: impl Into<String>) -> Self {
        self.expiry_date = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLotsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLotsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1InventoryLotsUpdateRequest, BuildError> {
        Ok(PostV1InventoryLotsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            expiry_date: self.expiry_date,
            notes: self.notes,
        })
    }
}
