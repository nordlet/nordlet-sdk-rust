pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionBomsGetResponseLinesItem {
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

impl PostV1ProductionBomsGetResponseLinesItem {
    pub fn builder() -> PostV1ProductionBomsGetResponseLinesItemBuilder {
        <PostV1ProductionBomsGetResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsGetResponseLinesItemBuilder {
    id: Option<String>,
    component_item_id: Option<String>,
    quantity: Option<String>,
    scrap_percent: Option<String>,
}

impl PostV1ProductionBomsGetResponseLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ProductionBomsGetResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionBomsGetResponseLinesItemBuilder::id)
    /// - [`component_item_id`](PostV1ProductionBomsGetResponseLinesItemBuilder::component_item_id)
    /// - [`quantity`](PostV1ProductionBomsGetResponseLinesItemBuilder::quantity)
    /// - [`scrap_percent`](PostV1ProductionBomsGetResponseLinesItemBuilder::scrap_percent)
    pub fn build(self) -> Result<PostV1ProductionBomsGetResponseLinesItem, BuildError> {
        Ok(PostV1ProductionBomsGetResponseLinesItem {
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
