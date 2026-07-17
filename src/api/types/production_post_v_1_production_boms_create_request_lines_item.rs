pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionBomsCreateRequestLinesItem {
    #[serde(rename = "componentItemId")]
    #[serde(default)]
    pub component_item_id: String,
    #[serde(default)]
    pub quantity: String,
}

impl PostV1ProductionBomsCreateRequestLinesItem {
    pub fn builder() -> PostV1ProductionBomsCreateRequestLinesItemBuilder {
        <PostV1ProductionBomsCreateRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsCreateRequestLinesItemBuilder {
    component_item_id: Option<String>,
    quantity: Option<String>,
}

impl PostV1ProductionBomsCreateRequestLinesItemBuilder {
    pub fn component_item_id(mut self, value: impl Into<String>) -> Self {
        self.component_item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionBomsCreateRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`component_item_id`](PostV1ProductionBomsCreateRequestLinesItemBuilder::component_item_id)
    /// - [`quantity`](PostV1ProductionBomsCreateRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1ProductionBomsCreateRequestLinesItem, BuildError> {
        Ok(PostV1ProductionBomsCreateRequestLinesItem {
            component_item_id: self
                .component_item_id
                .ok_or_else(|| BuildError::missing_field("component_item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
