pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaybillsCreateTransportRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
}

impl WaybillsCreateTransportRequestLinesItem {
    pub fn builder() -> WaybillsCreateTransportRequestLinesItemBuilder {
        <WaybillsCreateTransportRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaybillsCreateTransportRequestLinesItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<String>,
    product_code: Option<String>,
}

impl WaybillsCreateTransportRequestLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`WaybillsCreateTransportRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](WaybillsCreateTransportRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<WaybillsCreateTransportRequestLinesItem, BuildError> {
        Ok(WaybillsCreateTransportRequestLinesItem {
            item_id: self.item_id,
            description: self.description,
            unit: self.unit,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            product_code: self.product_code,
        })
    }
}
