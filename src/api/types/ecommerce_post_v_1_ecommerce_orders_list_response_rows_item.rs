pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersListResponseRowsItem {
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
    pub status: PostV1EcommerceOrdersListResponseRowsItemStatus,
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
}

impl PostV1EcommerceOrdersListResponseRowsItem {
    pub fn builder() -> PostV1EcommerceOrdersListResponseRowsItemBuilder {
        <PostV1EcommerceOrdersListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersListResponseRowsItemBuilder {
    id: Option<String>,
    channel: Option<String>,
    external_ref: Option<String>,
    partner_id: Option<String>,
    warehouse_id: Option<String>,
    currency: Option<String>,
    status: Option<PostV1EcommerceOrdersListResponseRowsItemStatus>,
    invoice_id: Option<String>,
    ship_to_country_code: Option<String>,
    marketplace: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1EcommerceOrdersListResponseRowsItemBuilder {
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

    pub fn status(mut self, value: PostV1EcommerceOrdersListResponseRowsItemStatus) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1EcommerceOrdersListResponseRowsItemBuilder::id)
    /// - [`channel`](PostV1EcommerceOrdersListResponseRowsItemBuilder::channel)
    /// - [`partner_id`](PostV1EcommerceOrdersListResponseRowsItemBuilder::partner_id)
    /// - [`currency`](PostV1EcommerceOrdersListResponseRowsItemBuilder::currency)
    /// - [`status`](PostV1EcommerceOrdersListResponseRowsItemBuilder::status)
    /// - [`created_at`](PostV1EcommerceOrdersListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1EcommerceOrdersListResponseRowsItem, BuildError> {
        Ok(PostV1EcommerceOrdersListResponseRowsItem {
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
        })
    }
}
