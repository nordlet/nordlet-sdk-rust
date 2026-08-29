pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ProductionOrdersCompleteResponse {
    #[serde(default)]
    pub id: String,
    pub r#type: PostV1ProductionOrdersCompleteResponseType,
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
    pub date: String,
    pub status: PostV1ProductionOrdersCompleteResponseStatus,
    #[serde(rename = "scrappedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrapped_quantity: Option<String>,
    #[serde(rename = "materialCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_cost: Option<String>,
    #[serde(rename = "laborCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labor_cost: Option<String>,
    #[serde(rename = "scrapCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrap_cost: Option<String>,
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

impl PostV1ProductionOrdersCompleteResponse {
    pub fn builder() -> PostV1ProductionOrdersCompleteResponseBuilder {
        <PostV1ProductionOrdersCompleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersCompleteResponseBuilder {
    id: Option<String>,
    r#type: Option<PostV1ProductionOrdersCompleteResponseType>,
    bom_id: Option<String>,
    warehouse_id: Option<String>,
    routing_id: Option<String>,
    quantity: Option<String>,
    date: Option<String>,
    status: Option<PostV1ProductionOrdersCompleteResponseStatus>,
    scrapped_quantity: Option<String>,
    material_cost: Option<String>,
    labor_cost: Option<String>,
    scrap_cost: Option<String>,
    total_cost: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1ProductionOrdersCompleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1ProductionOrdersCompleteResponseType) -> Self {
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

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1ProductionOrdersCompleteResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn scrapped_quantity(mut self, value: impl Into<String>) -> Self {
        self.scrapped_quantity = Some(value.into());
        self
    }

    pub fn material_cost(mut self, value: impl Into<String>) -> Self {
        self.material_cost = Some(value.into());
        self
    }

    pub fn labor_cost(mut self, value: impl Into<String>) -> Self {
        self.labor_cost = Some(value.into());
        self
    }

    pub fn scrap_cost(mut self, value: impl Into<String>) -> Self {
        self.scrap_cost = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersCompleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionOrdersCompleteResponseBuilder::id)
    /// - [`r#type`](PostV1ProductionOrdersCompleteResponseBuilder::r#type)
    /// - [`bom_id`](PostV1ProductionOrdersCompleteResponseBuilder::bom_id)
    /// - [`warehouse_id`](PostV1ProductionOrdersCompleteResponseBuilder::warehouse_id)
    /// - [`quantity`](PostV1ProductionOrdersCompleteResponseBuilder::quantity)
    /// - [`date`](PostV1ProductionOrdersCompleteResponseBuilder::date)
    /// - [`status`](PostV1ProductionOrdersCompleteResponseBuilder::status)
    /// - [`created_at`](PostV1ProductionOrdersCompleteResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1ProductionOrdersCompleteResponse, BuildError> {
        Ok(PostV1ProductionOrdersCompleteResponse {
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
            routing_id: self.routing_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            scrapped_quantity: self.scrapped_quantity,
            material_cost: self.material_cost,
            labor_cost: self.labor_cost,
            scrap_cost: self.scrap_cost,
            total_cost: self.total_cost,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
