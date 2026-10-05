pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReorderRulesUpdateInventoryRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "minQty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_qty: Option<String>,
    #[serde(rename = "reorderQty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_qty: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ReorderRulesUpdateInventoryRequest {
    pub fn builder() -> ReorderRulesUpdateInventoryRequestBuilder {
        <ReorderRulesUpdateInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesUpdateInventoryRequestBuilder {
    id: Option<String>,
    min_qty: Option<String>,
    reorder_qty: Option<String>,
    is_active: Option<bool>,
    notes: Option<String>,
}

impl ReorderRulesUpdateInventoryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn min_qty(mut self, value: impl Into<String>) -> Self {
        self.min_qty = Some(value.into());
        self
    }

    pub fn reorder_qty(mut self, value: impl Into<String>) -> Self {
        self.reorder_qty = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReorderRulesUpdateInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReorderRulesUpdateInventoryRequestBuilder::id)
    pub fn build(self) -> Result<ReorderRulesUpdateInventoryRequest, BuildError> {
        Ok(ReorderRulesUpdateInventoryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            min_qty: self.min_qty,
            reorder_qty: self.reorder_qty,
            is_active: self.is_active,
            notes: self.notes,
        })
    }
}
