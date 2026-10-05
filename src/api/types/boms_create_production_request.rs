pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BomsCreateProductionRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "finishedItemId")]
    #[serde(default)]
    pub finished_item_id: String,
    #[serde(rename = "outputQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_quantity: Option<String>,
    #[serde(rename = "routingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_id: Option<String>,
    #[serde(default)]
    pub lines: Vec<BomsCreateProductionRequestLinesItem>,
}

impl BomsCreateProductionRequest {
    pub fn builder() -> BomsCreateProductionRequestBuilder {
        <BomsCreateProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsCreateProductionRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    finished_item_id: Option<String>,
    output_quantity: Option<String>,
    routing_id: Option<String>,
    lines: Option<Vec<BomsCreateProductionRequestLinesItem>>,
}

impl BomsCreateProductionRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn finished_item_id(mut self, value: impl Into<String>) -> Self {
        self.finished_item_id = Some(value.into());
        self
    }

    pub fn output_quantity(mut self, value: impl Into<String>) -> Self {
        self.output_quantity = Some(value.into());
        self
    }

    pub fn routing_id(mut self, value: impl Into<String>) -> Self {
        self.routing_id = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<BomsCreateProductionRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BomsCreateProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](BomsCreateProductionRequestBuilder::code)
    /// - [`name`](BomsCreateProductionRequestBuilder::name)
    /// - [`finished_item_id`](BomsCreateProductionRequestBuilder::finished_item_id)
    /// - [`lines`](BomsCreateProductionRequestBuilder::lines)
    pub fn build(self) -> Result<BomsCreateProductionRequest, BuildError> {
        Ok(BomsCreateProductionRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            finished_item_id: self
                .finished_item_id
                .ok_or_else(|| BuildError::missing_field("finished_item_id"))?,
            output_quantity: self.output_quantity,
            routing_id: self.routing_id,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
