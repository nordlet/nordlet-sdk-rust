pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ProductionQualityChecksListResponseRowsItem {
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
    pub result: PostV1ProductionQualityChecksListResponseRowsItemResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "checkedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_at: Option<String>,
    #[serde(rename = "checkedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_by: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1ProductionQualityChecksListResponseRowsItem {
    pub fn builder() -> PostV1ProductionQualityChecksListResponseRowsItemBuilder {
        <PostV1ProductionQualityChecksListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionQualityChecksListResponseRowsItemBuilder {
    id: Option<String>,
    order_id: Option<String>,
    routing_operation_id: Option<String>,
    name: Option<String>,
    result: Option<PostV1ProductionQualityChecksListResponseRowsItemResult>,
    notes: Option<String>,
    checked_at: Option<String>,
    checked_by: Option<String>,
    created_at: Option<String>,
}

impl PostV1ProductionQualityChecksListResponseRowsItemBuilder {
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

    pub fn result(
        mut self,
        value: PostV1ProductionQualityChecksListResponseRowsItemResult,
    ) -> Self {
        self.result = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn checked_at(mut self, value: impl Into<String>) -> Self {
        self.checked_at = Some(value.into());
        self
    }

    pub fn checked_by(mut self, value: impl Into<String>) -> Self {
        self.checked_by = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionQualityChecksListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionQualityChecksListResponseRowsItemBuilder::id)
    /// - [`order_id`](PostV1ProductionQualityChecksListResponseRowsItemBuilder::order_id)
    /// - [`name`](PostV1ProductionQualityChecksListResponseRowsItemBuilder::name)
    /// - [`result`](PostV1ProductionQualityChecksListResponseRowsItemBuilder::result)
    /// - [`created_at`](PostV1ProductionQualityChecksListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1ProductionQualityChecksListResponseRowsItem, BuildError> {
        Ok(PostV1ProductionQualityChecksListResponseRowsItem {
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
