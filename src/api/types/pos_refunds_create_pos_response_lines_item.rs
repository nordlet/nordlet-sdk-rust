pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsCreatePosResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "receiptLineId")]
    #[serde(default)]
    pub receipt_line_id: String,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "netAmount")]
    #[serde(default)]
    pub net_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
    #[serde(rename = "grossAmount")]
    #[serde(default)]
    pub gross_amount: String,
}

impl RefundsCreatePosResponseLinesItem {
    pub fn builder() -> RefundsCreatePosResponseLinesItemBuilder {
        <RefundsCreatePosResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsCreatePosResponseLinesItemBuilder {
    id: Option<String>,
    receipt_line_id: Option<String>,
    item_id: Option<String>,
    quantity: Option<String>,
    net_amount: Option<String>,
    vat_amount: Option<String>,
    gross_amount: Option<String>,
}

impl RefundsCreatePosResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn receipt_line_id(mut self, value: impl Into<String>) -> Self {
        self.receipt_line_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn net_amount(mut self, value: impl Into<String>) -> Self {
        self.net_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    pub fn gross_amount(mut self, value: impl Into<String>) -> Self {
        self.gross_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefundsCreatePosResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundsCreatePosResponseLinesItemBuilder::id)
    /// - [`receipt_line_id`](RefundsCreatePosResponseLinesItemBuilder::receipt_line_id)
    /// - [`quantity`](RefundsCreatePosResponseLinesItemBuilder::quantity)
    /// - [`net_amount`](RefundsCreatePosResponseLinesItemBuilder::net_amount)
    /// - [`vat_amount`](RefundsCreatePosResponseLinesItemBuilder::vat_amount)
    /// - [`gross_amount`](RefundsCreatePosResponseLinesItemBuilder::gross_amount)
    pub fn build(self) -> Result<RefundsCreatePosResponseLinesItem, BuildError> {
        Ok(RefundsCreatePosResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            receipt_line_id: self
                .receipt_line_id
                .ok_or_else(|| BuildError::missing_field("receipt_line_id"))?,
            item_id: self.item_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
            gross_amount: self
                .gross_amount
                .ok_or_else(|| BuildError::missing_field("gross_amount"))?,
        })
    }
}
