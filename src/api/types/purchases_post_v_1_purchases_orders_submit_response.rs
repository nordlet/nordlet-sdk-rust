pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersSubmitResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    pub status: PostV1PurchasesOrdersSubmitResponseStatus,
    #[serde(rename = "orderNumber")]
    #[serde(default)]
    pub order_number: String,
    #[serde(rename = "orderDate")]
    #[serde(default)]
    pub order_date: String,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<String>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "approvedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    #[serde(rename = "approvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub lines: Vec<PostV1PurchasesOrdersSubmitResponseLinesItem>,
}

impl PostV1PurchasesOrdersSubmitResponse {
    pub fn builder() -> PostV1PurchasesOrdersSubmitResponseBuilder {
        <PostV1PurchasesOrdersSubmitResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersSubmitResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    status: Option<PostV1PurchasesOrdersSubmitResponseStatus>,
    order_number: Option<String>,
    order_date: Option<String>,
    expected_date: Option<String>,
    warehouse_id: Option<String>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    approved_by: Option<String>,
    approved_at: Option<String>,
    notes: Option<String>,
    document_ref: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    lines: Option<Vec<PostV1PurchasesOrdersSubmitResponseLinesItem>>,
}

impl PostV1PurchasesOrdersSubmitResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1PurchasesOrdersSubmitResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn order_number(mut self, value: impl Into<String>) -> Self {
        self.order_number = Some(value.into());
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

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn approved_by(mut self, value: impl Into<String>) -> Self {
        self.approved_by = Some(value.into());
        self
    }

    pub fn approved_at(mut self, value: impl Into<String>) -> Self {
        self.approved_at = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1PurchasesOrdersSubmitResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersSubmitResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersSubmitResponseBuilder::id)
    /// - [`partner_id`](PostV1PurchasesOrdersSubmitResponseBuilder::partner_id)
    /// - [`status`](PostV1PurchasesOrdersSubmitResponseBuilder::status)
    /// - [`order_number`](PostV1PurchasesOrdersSubmitResponseBuilder::order_number)
    /// - [`order_date`](PostV1PurchasesOrdersSubmitResponseBuilder::order_date)
    /// - [`currency`](PostV1PurchasesOrdersSubmitResponseBuilder::currency)
    /// - [`net_total`](PostV1PurchasesOrdersSubmitResponseBuilder::net_total)
    /// - [`vat_total`](PostV1PurchasesOrdersSubmitResponseBuilder::vat_total)
    /// - [`gross_total`](PostV1PurchasesOrdersSubmitResponseBuilder::gross_total)
    /// - [`created_at`](PostV1PurchasesOrdersSubmitResponseBuilder::created_at)
    /// - [`updated_at`](PostV1PurchasesOrdersSubmitResponseBuilder::updated_at)
    /// - [`lines`](PostV1PurchasesOrdersSubmitResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1PurchasesOrdersSubmitResponse, BuildError> {
        Ok(PostV1PurchasesOrdersSubmitResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            order_number: self
                .order_number
                .ok_or_else(|| BuildError::missing_field("order_number"))?,
            order_date: self
                .order_date
                .ok_or_else(|| BuildError::missing_field("order_date"))?,
            expected_date: self.expected_date,
            warehouse_id: self.warehouse_id,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            approved_by: self.approved_by,
            approved_at: self.approved_at,
            notes: self.notes,
            document_ref: self.document_ref,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
