pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BomsGetProductionResponseLinesItem {
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

impl BomsGetProductionResponseLinesItem {
    pub fn builder() -> BomsGetProductionResponseLinesItemBuilder {
        <BomsGetProductionResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsGetProductionResponseLinesItemBuilder {
    id: Option<String>,
    component_item_id: Option<String>,
    quantity: Option<String>,
    scrap_percent: Option<String>,
}

impl BomsGetProductionResponseLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`BomsGetProductionResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BomsGetProductionResponseLinesItemBuilder::id)
    /// - [`component_item_id`](BomsGetProductionResponseLinesItemBuilder::component_item_id)
    /// - [`quantity`](BomsGetProductionResponseLinesItemBuilder::quantity)
    /// - [`scrap_percent`](BomsGetProductionResponseLinesItemBuilder::scrap_percent)
    pub fn build(self) -> Result<BomsGetProductionResponseLinesItem, BuildError> {
        Ok(BomsGetProductionResponseLinesItem {
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
