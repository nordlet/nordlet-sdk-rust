pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1TransportWaybillsUpdateResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
}

impl PostV1TransportWaybillsUpdateResponseLinesItem {
    pub fn builder() -> PostV1TransportWaybillsUpdateResponseLinesItemBuilder {
        <PostV1TransportWaybillsUpdateResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsUpdateResponseLinesItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<String>,
    product_code: Option<String>,
    sort_order: Option<i64>,
}

impl PostV1TransportWaybillsUpdateResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn product_code(mut self, value: impl Into<String>) -> Self {
        self.product_code = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsUpdateResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1TransportWaybillsUpdateResponseLinesItemBuilder::id)
    /// - [`description`](PostV1TransportWaybillsUpdateResponseLinesItemBuilder::description)
    /// - [`unit`](PostV1TransportWaybillsUpdateResponseLinesItemBuilder::unit)
    /// - [`quantity`](PostV1TransportWaybillsUpdateResponseLinesItemBuilder::quantity)
    /// - [`sort_order`](PostV1TransportWaybillsUpdateResponseLinesItemBuilder::sort_order)
    pub fn build(self) -> Result<PostV1TransportWaybillsUpdateResponseLinesItem, BuildError> {
        Ok(PostV1TransportWaybillsUpdateResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            product_code: self.product_code,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
        })
    }
}
