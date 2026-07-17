pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "externalRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<PostV1EcommerceOrdersCreateRequestPartner>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "shipToCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_to_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1EcommerceOrdersCreateRequestLinesItem>,
}

impl PostV1EcommerceOrdersCreateRequest {
    pub fn builder() -> PostV1EcommerceOrdersCreateRequestBuilder {
        <PostV1EcommerceOrdersCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersCreateRequestBuilder {
    channel: Option<String>,
    external_ref: Option<String>,
    partner_id: Option<String>,
    partner: Option<PostV1EcommerceOrdersCreateRequestPartner>,
    warehouse_id: Option<String>,
    currency: Option<String>,
    ship_to_country_code: Option<String>,
    marketplace: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<PostV1EcommerceOrdersCreateRequestLinesItem>>,
}

impl PostV1EcommerceOrdersCreateRequestBuilder {
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

    pub fn partner(mut self, value: PostV1EcommerceOrdersCreateRequestPartner) -> Self {
        self.partner = Some(value);
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

    pub fn lines(mut self, value: Vec<PostV1EcommerceOrdersCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lines`](PostV1EcommerceOrdersCreateRequestBuilder::lines)
    pub fn build(self) -> Result<PostV1EcommerceOrdersCreateRequest, BuildError> {
        Ok(PostV1EcommerceOrdersCreateRequest {
            channel: self.channel,
            external_ref: self.external_ref,
            partner_id: self.partner_id,
            partner: self.partner,
            warehouse_id: self.warehouse_id,
            currency: self.currency,
            ship_to_country_code: self.ship_to_country_code,
            marketplace: self.marketplace,
            notes: self.notes,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
