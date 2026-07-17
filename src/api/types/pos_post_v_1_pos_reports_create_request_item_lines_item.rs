pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsCreateRequestItemLinesItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub quantity: String,
}

impl PostV1PosReportsCreateRequestItemLinesItem {
    pub fn builder() -> PostV1PosReportsCreateRequestItemLinesItemBuilder {
        <PostV1PosReportsCreateRequestItemLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsCreateRequestItemLinesItemBuilder {
    item_id: Option<String>,
    quantity: Option<String>,
}

impl PostV1PosReportsCreateRequestItemLinesItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosReportsCreateRequestItemLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1PosReportsCreateRequestItemLinesItemBuilder::item_id)
    /// - [`quantity`](PostV1PosReportsCreateRequestItemLinesItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1PosReportsCreateRequestItemLinesItem, BuildError> {
        Ok(PostV1PosReportsCreateRequestItemLinesItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
