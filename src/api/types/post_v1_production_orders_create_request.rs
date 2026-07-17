pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionOrdersCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1ProductionOrdersCreateRequestType>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProductionOrdersCreateRequest {
    pub fn builder() -> PostV1ProductionOrdersCreateRequestBuilder {
        <PostV1ProductionOrdersCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersCreateRequestBuilder {
    r#type: Option<PostV1ProductionOrdersCreateRequestType>,
    bom_id: Option<String>,
    warehouse_id: Option<String>,
    quantity: Option<String>,
    date: Option<String>,
    notes: Option<String>,
}

impl PostV1ProductionOrdersCreateRequestBuilder {
    pub fn r#type(mut self, value: PostV1ProductionOrdersCreateRequestType) -> Self {
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bom_id`](PostV1ProductionOrdersCreateRequestBuilder::bom_id)
    /// - [`warehouse_id`](PostV1ProductionOrdersCreateRequestBuilder::warehouse_id)
    /// - [`quantity`](PostV1ProductionOrdersCreateRequestBuilder::quantity)
    /// - [`date`](PostV1ProductionOrdersCreateRequestBuilder::date)
    pub fn build(self) -> Result<PostV1ProductionOrdersCreateRequest, BuildError> {
        Ok(PostV1ProductionOrdersCreateRequest {
            r#type: self.r#type,
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
            notes: self.notes,
        })
    }
}
