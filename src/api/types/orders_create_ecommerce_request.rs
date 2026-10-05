pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersCreateEcommerceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "externalRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<OrdersCreateEcommerceRequestPartner>,
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
    pub lines: Vec<OrdersCreateEcommerceRequestLinesItem>,
}

impl OrdersCreateEcommerceRequest {
    pub fn builder() -> OrdersCreateEcommerceRequestBuilder {
        <OrdersCreateEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersCreateEcommerceRequestBuilder {
    channel: Option<String>,
    external_ref: Option<String>,
    partner_id: Option<String>,
    partner: Option<OrdersCreateEcommerceRequestPartner>,
    warehouse_id: Option<String>,
    currency: Option<String>,
    ship_to_country_code: Option<String>,
    marketplace: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<OrdersCreateEcommerceRequestLinesItem>>,
}

impl OrdersCreateEcommerceRequestBuilder {
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

    pub fn partner(mut self, value: OrdersCreateEcommerceRequestPartner) -> Self {
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

    pub fn lines(mut self, value: Vec<OrdersCreateEcommerceRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersCreateEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lines`](OrdersCreateEcommerceRequestBuilder::lines)
    pub fn build(self) -> Result<OrdersCreateEcommerceRequest, BuildError> {
        Ok(OrdersCreateEcommerceRequest {
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
