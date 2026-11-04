pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsCreatePosRequestLinesItem {
    #[serde(rename = "receiptLineId")]
    #[serde(default)]
    pub receipt_line_id: String,
    #[serde(default)]
    pub quantity: String,
}

impl RefundsCreatePosRequestLinesItem {
    pub fn builder() -> RefundsCreatePosRequestLinesItemBuilder {
        <RefundsCreatePosRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsCreatePosRequestLinesItemBuilder {
    receipt_line_id: Option<String>,
    quantity: Option<String>,
}

impl RefundsCreatePosRequestLinesItemBuilder {
    pub fn receipt_line_id(mut self, value: impl Into<String>) -> Self {
        self.receipt_line_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefundsCreatePosRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`receipt_line_id`](RefundsCreatePosRequestLinesItemBuilder::receipt_line_id)
    /// - [`quantity`](RefundsCreatePosRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<RefundsCreatePosRequestLinesItem, BuildError> {
        Ok(RefundsCreatePosRequestLinesItem {
            receipt_line_id: self
                .receipt_line_id
                .ok_or_else(|| BuildError::missing_field("receipt_line_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
