pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BomsCreateProductionRequestLinesItem {
    #[serde(rename = "componentItemId")]
    #[serde(default)]
    pub component_item_id: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "scrapPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrap_percent: Option<String>,
}

impl BomsCreateProductionRequestLinesItem {
    pub fn builder() -> BomsCreateProductionRequestLinesItemBuilder {
        <BomsCreateProductionRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsCreateProductionRequestLinesItemBuilder {
    component_item_id: Option<String>,
    quantity: Option<String>,
    scrap_percent: Option<String>,
}

impl BomsCreateProductionRequestLinesItemBuilder {
    pub fn component_item_id(mut self, value: impl Into<String>) -> Self {
        self.component_item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn scrap_percent(mut self, value: impl Into<String>) -> Self {
        self.scrap_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BomsCreateProductionRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`component_item_id`](BomsCreateProductionRequestLinesItemBuilder::component_item_id)
    /// - [`quantity`](BomsCreateProductionRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<BomsCreateProductionRequestLinesItem, BuildError> {
        Ok(BomsCreateProductionRequestLinesItem {
            component_item_id: self
                .component_item_id
                .ok_or_else(|| BuildError::missing_field("component_item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            scrap_percent: self.scrap_percent,
        })
    }
}
