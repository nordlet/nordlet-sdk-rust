pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1PurchasesOrdersUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "orderDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_date: Option<String>,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<String>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostV1PurchasesOrdersUpdateRequestLinesItem>>,
}

impl PostV1PurchasesOrdersUpdateRequest {
    pub fn builder() -> PostV1PurchasesOrdersUpdateRequestBuilder {
        <PostV1PurchasesOrdersUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersUpdateRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    order_date: Option<String>,
    expected_date: Option<String>,
    warehouse_id: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<PostV1PurchasesOrdersUpdateRequestLinesItem>>,
}

impl PostV1PurchasesOrdersUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn order_date(mut self, value: impl Into<String>) -> Self {
        self.order_date = Some(value.into());
        self
    }

    pub fn expected_date(mut self, value: impl Into<String>) -> Self {
        self.expected_date = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1PurchasesOrdersUpdateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesOrdersUpdateRequest, BuildError> {
        Ok(PostV1PurchasesOrdersUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            order_date: self.order_date,
            expected_date: self.expected_date,
            warehouse_id: self.warehouse_id,
            currency: self.currency,
            notes: self.notes,
            lines: self.lines,
        })
    }
}
