pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersRecordOperationProductionResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "routingOperationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_operation_id: Option<String>,
    #[serde(rename = "workCenterId")]
    #[serde(default)]
    pub work_center_id: String,
    #[serde(default)]
    pub sequence: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "plannedMinutes")]
    #[serde(default)]
    pub planned_minutes: String,
    #[serde(rename = "actualMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_minutes: Option<String>,
    #[serde(rename = "costPerHour")]
    #[serde(default)]
    pub cost_per_hour: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
}

impl OrdersRecordOperationProductionResponse {
    pub fn builder() -> OrdersRecordOperationProductionResponseBuilder {
        <OrdersRecordOperationProductionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersRecordOperationProductionResponseBuilder {
    id: Option<String>,
    routing_operation_id: Option<String>,
    work_center_id: Option<String>,
    sequence: Option<i64>,
    name: Option<String>,
    planned_minutes: Option<String>,
    actual_minutes: Option<String>,
    cost_per_hour: Option<String>,
    cost: Option<String>,
}

impl OrdersRecordOperationProductionResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn routing_operation_id(mut self, value: impl Into<String>) -> Self {
        self.routing_operation_id = Some(value.into());
        self
    }

    pub fn work_center_id(mut self, value: impl Into<String>) -> Self {
        self.work_center_id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn planned_minutes(mut self, value: impl Into<String>) -> Self {
        self.planned_minutes = Some(value.into());
        self
    }

    pub fn actual_minutes(mut self, value: impl Into<String>) -> Self {
        self.actual_minutes = Some(value.into());
        self
    }

    pub fn cost_per_hour(mut self, value: impl Into<String>) -> Self {
        self.cost_per_hour = Some(value.into());
        self
    }

    pub fn cost(mut self, value: impl Into<String>) -> Self {
        self.cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersRecordOperationProductionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersRecordOperationProductionResponseBuilder::id)
    /// - [`work_center_id`](OrdersRecordOperationProductionResponseBuilder::work_center_id)
    /// - [`sequence`](OrdersRecordOperationProductionResponseBuilder::sequence)
    /// - [`name`](OrdersRecordOperationProductionResponseBuilder::name)
    /// - [`planned_minutes`](OrdersRecordOperationProductionResponseBuilder::planned_minutes)
    /// - [`cost_per_hour`](OrdersRecordOperationProductionResponseBuilder::cost_per_hour)
    pub fn build(self) -> Result<OrdersRecordOperationProductionResponse, BuildError> {
        Ok(OrdersRecordOperationProductionResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            routing_operation_id: self.routing_operation_id,
            work_center_id: self
                .work_center_id
                .ok_or_else(|| BuildError::missing_field("work_center_id"))?,
            sequence: self
                .sequence
                .ok_or_else(|| BuildError::missing_field("sequence"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            planned_minutes: self
                .planned_minutes
                .ok_or_else(|| BuildError::missing_field("planned_minutes"))?,
            actual_minutes: self.actual_minutes,
            cost_per_hour: self
                .cost_per_hour
                .ok_or_else(|| BuildError::missing_field("cost_per_hour"))?,
            cost: self.cost,
        })
    }
}
