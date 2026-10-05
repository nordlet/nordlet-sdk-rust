pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersCreateProductionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OrdersCreateProductionRequestType>,
    #[serde(rename = "bomId")]
    #[serde(default)]
    pub bom_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "routingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_id: Option<String>,
    #[serde(default)]
    pub quantity: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl OrdersCreateProductionRequest {
    pub fn builder() -> OrdersCreateProductionRequestBuilder {
        <OrdersCreateProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersCreateProductionRequestBuilder {
    r#type: Option<OrdersCreateProductionRequestType>,
    bom_id: Option<String>,
    warehouse_id: Option<String>,
    routing_id: Option<String>,
    quantity: Option<String>,
    date: Option<NaiveDate>,
    notes: Option<String>,
}

impl OrdersCreateProductionRequestBuilder {
    pub fn r#type(mut self, value: OrdersCreateProductionRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn bom_id(mut self, value: impl Into<String>) -> Self {
        self.bom_id = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn routing_id(mut self, value: impl Into<String>) -> Self {
        self.routing_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersCreateProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bom_id`](OrdersCreateProductionRequestBuilder::bom_id)
    /// - [`warehouse_id`](OrdersCreateProductionRequestBuilder::warehouse_id)
    /// - [`quantity`](OrdersCreateProductionRequestBuilder::quantity)
    /// - [`date`](OrdersCreateProductionRequestBuilder::date)
    pub fn build(self) -> Result<OrdersCreateProductionRequest, BuildError> {
        Ok(OrdersCreateProductionRequest {
            r#type: self.r#type,
            bom_id: self
                .bom_id
                .ok_or_else(|| BuildError::missing_field("bom_id"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            routing_id: self.routing_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            notes: self.notes,
        })
    }
}
