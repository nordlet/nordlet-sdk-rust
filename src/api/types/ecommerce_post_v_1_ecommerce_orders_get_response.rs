pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "externalRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(default)]
    pub currency: String,
    pub status: PostV1EcommerceOrdersGetResponseStatus,
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(rename = "shipToCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_to_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub lines: Vec<PostV1EcommerceOrdersGetResponseLinesItem>,
}

impl PostV1EcommerceOrdersGetResponse {
    pub fn builder() -> PostV1EcommerceOrdersGetResponseBuilder {
        <PostV1EcommerceOrdersGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersGetResponseBuilder {
    id: Option<String>,
    channel: Option<String>,
    external_ref: Option<String>,
    partner_id: Option<String>,
    warehouse_id: Option<String>,
    currency: Option<String>,
    status: Option<PostV1EcommerceOrdersGetResponseStatus>,
    invoice_id: Option<String>,
    ship_to_country_code: Option<String>,
    marketplace: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    lines: Option<Vec<PostV1EcommerceOrdersGetResponseLinesItem>>,
}

impl PostV1EcommerceOrdersGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn external_ref(mut self, value: impl Into<String>) -> Self {
        self.external_ref = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
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

    pub fn status(mut self, value: PostV1EcommerceOrdersGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn ship_to_country_code(mut self, value: impl Into<String>) -> Self {
        self.ship_to_country_code = Some(value.into());
        self
    }

    pub fn marketplace(mut self, value: impl Into<String>) -> Self {
        self.marketplace = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1EcommerceOrdersGetResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1EcommerceOrdersGetResponseBuilder::id)
    /// - [`channel`](PostV1EcommerceOrdersGetResponseBuilder::channel)
    /// - [`partner_id`](PostV1EcommerceOrdersGetResponseBuilder::partner_id)
    /// - [`currency`](PostV1EcommerceOrdersGetResponseBuilder::currency)
    /// - [`status`](PostV1EcommerceOrdersGetResponseBuilder::status)
    /// - [`created_at`](PostV1EcommerceOrdersGetResponseBuilder::created_at)
    /// - [`lines`](PostV1EcommerceOrdersGetResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1EcommerceOrdersGetResponse, BuildError> {
        Ok(PostV1EcommerceOrdersGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            channel: self
                .channel
                .ok_or_else(|| BuildError::missing_field("channel"))?,
            external_ref: self.external_ref,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            warehouse_id: self.warehouse_id,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            invoice_id: self.invoice_id,
            ship_to_country_code: self.ship_to_country_code,
            marketplace: self.marketplace,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
