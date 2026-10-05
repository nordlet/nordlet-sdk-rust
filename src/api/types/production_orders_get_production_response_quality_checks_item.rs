pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrdersGetProductionResponseQualityChecksItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(rename = "routingOperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_operation_id: Option<String>,
    #[serde(default)]
    pub name: String,
    pub result: OrdersGetProductionResponseQualityChecksItemResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "checkedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub checked_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "checkedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_by: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl OrdersGetProductionResponseQualityChecksItem {
    pub fn builder() -> OrdersGetProductionResponseQualityChecksItemBuilder {
        <OrdersGetProductionResponseQualityChecksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersGetProductionResponseQualityChecksItemBuilder {
    id: Option<String>,
    order_id: Option<String>,
    routing_operation_id: Option<String>,
    name: Option<String>,
    result: Option<OrdersGetProductionResponseQualityChecksItemResult>,
    notes: Option<String>,
    checked_at: Option<DateTime<FixedOffset>>,
    checked_by: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl OrdersGetProductionResponseQualityChecksItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn routing_operation_id(mut self, value: impl Into<String>) -> Self {
        self.routing_operation_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn result(mut self, value: OrdersGetProductionResponseQualityChecksItemResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn checked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.checked_at = Some(value);
        self
    }

    pub fn checked_by(mut self, value: impl Into<String>) -> Self {
        self.checked_by = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersGetProductionResponseQualityChecksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersGetProductionResponseQualityChecksItemBuilder::id)
    /// - [`order_id`](OrdersGetProductionResponseQualityChecksItemBuilder::order_id)
    /// - [`name`](OrdersGetProductionResponseQualityChecksItemBuilder::name)
    /// - [`result`](OrdersGetProductionResponseQualityChecksItemBuilder::result)
    /// - [`created_at`](OrdersGetProductionResponseQualityChecksItemBuilder::created_at)
    pub fn build(self) -> Result<OrdersGetProductionResponseQualityChecksItem, BuildError> {
        Ok(OrdersGetProductionResponseQualityChecksItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            routing_operation_id: self.routing_operation_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            notes: self.notes,
            checked_at: self.checked_at,
            checked_by: self.checked_by,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
