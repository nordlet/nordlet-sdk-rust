pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BomsCreateProductionResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "componentItemId")]
    #[serde(default)]
    pub component_item_id: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "scrapPercent")]
    #[serde(default)]
    pub scrap_percent: String,
}

impl BomsCreateProductionResponseLinesItem {
    pub fn builder() -> BomsCreateProductionResponseLinesItemBuilder {
        <BomsCreateProductionResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsCreateProductionResponseLinesItemBuilder {
    id: Option<String>,
    component_item_id: Option<String>,
    quantity: Option<String>,
    scrap_percent: Option<String>,
}

impl BomsCreateProductionResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`BomsCreateProductionResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BomsCreateProductionResponseLinesItemBuilder::id)
    /// - [`component_item_id`](BomsCreateProductionResponseLinesItemBuilder::component_item_id)
    /// - [`quantity`](BomsCreateProductionResponseLinesItemBuilder::quantity)
    /// - [`scrap_percent`](BomsCreateProductionResponseLinesItemBuilder::scrap_percent)
    pub fn build(self) -> Result<BomsCreateProductionResponseLinesItem, BuildError> {
        Ok(BomsCreateProductionResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            component_item_id: self
                .component_item_id
                .ok_or_else(|| BuildError::missing_field("component_item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            scrap_percent: self
                .scrap_percent
                .ok_or_else(|| BuildError::missing_field("scrap_percent"))?,
        })
    }
}
