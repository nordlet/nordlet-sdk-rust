pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterItemsReportsResponseRowsItem {
    #[serde(rename = "costCenterCode")]
    #[serde(default)]
    pub cost_center_code: String,
    #[serde(rename = "costCenterName")]
    #[serde(default)]
    pub cost_center_name: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub net: String,
}

impl CostCenterItemsReportsResponseRowsItem {
    pub fn builder() -> CostCenterItemsReportsResponseRowsItemBuilder {
        <CostCenterItemsReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterItemsReportsResponseRowsItemBuilder {
    cost_center_code: Option<String>,
    cost_center_name: Option<String>,
    item_name: Option<String>,
    net: Option<String>,
}

impl CostCenterItemsReportsResponseRowsItemBuilder {
    pub fn cost_center_code(mut self, value: impl Into<String>) -> Self {
        self.cost_center_code = Some(value.into());
        self
    }

    pub fn cost_center_name(mut self, value: impl Into<String>) -> Self {
        self.cost_center_name = Some(value.into());
        self
    }

    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.item_name = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCenterItemsReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cost_center_code`](CostCenterItemsReportsResponseRowsItemBuilder::cost_center_code)
    /// - [`cost_center_name`](CostCenterItemsReportsResponseRowsItemBuilder::cost_center_name)
    /// - [`item_name`](CostCenterItemsReportsResponseRowsItemBuilder::item_name)
    /// - [`net`](CostCenterItemsReportsResponseRowsItemBuilder::net)
    pub fn build(self) -> Result<CostCenterItemsReportsResponseRowsItem, BuildError> {
        Ok(CostCenterItemsReportsResponseRowsItem {
            cost_center_code: self
                .cost_center_code
                .ok_or_else(|| BuildError::missing_field("cost_center_code"))?,
            cost_center_name: self
                .cost_center_name
                .ok_or_else(|| BuildError::missing_field("cost_center_name"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
