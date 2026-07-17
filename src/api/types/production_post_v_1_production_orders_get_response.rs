pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ProductionOrdersGetResponse {
    #[serde(default)]
    pub id: String,
    pub r#type: PostV1ProductionOrdersGetResponseType,
    #[serde(rename = "bomId")]
    #[serde(default)]
    pub bom_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(default)]
    pub date: String,
    pub status: PostV1ProductionOrdersGetResponseStatus,
    #[serde(rename = "totalCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<String>,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1ProductionOrdersGetResponse {
    pub fn builder() -> PostV1ProductionOrdersGetResponseBuilder {
        <PostV1ProductionOrdersGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersGetResponseBuilder {
    id: Option<String>,
    r#type: Option<PostV1ProductionOrdersGetResponseType>,
    bom_id: Option<String>,
    warehouse_id: Option<String>,
    quantity: Option<String>,
    date: Option<String>,
    status: Option<PostV1ProductionOrdersGetResponseStatus>,
    total_cost: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1ProductionOrdersGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1ProductionOrdersGetResponseType) -> Self {
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

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1ProductionOrdersGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionOrdersGetResponseBuilder::id)
    /// - [`r#type`](PostV1ProductionOrdersGetResponseBuilder::r#type)
    /// - [`bom_id`](PostV1ProductionOrdersGetResponseBuilder::bom_id)
    /// - [`warehouse_id`](PostV1ProductionOrdersGetResponseBuilder::warehouse_id)
    /// - [`quantity`](PostV1ProductionOrdersGetResponseBuilder::quantity)
    /// - [`date`](PostV1ProductionOrdersGetResponseBuilder::date)
    /// - [`status`](PostV1ProductionOrdersGetResponseBuilder::status)
    /// - [`created_at`](PostV1ProductionOrdersGetResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1ProductionOrdersGetResponse, BuildError> {
        Ok(PostV1ProductionOrdersGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            bom_id: self
                .bom_id
                .ok_or_else(|| BuildError::missing_field("bom_id"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total_cost: self.total_cost,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
