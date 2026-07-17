pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCostCenterItemsResponseRowsItem {
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

impl PostV1ReportsCostCenterItemsResponseRowsItem {
    pub fn builder() -> PostV1ReportsCostCenterItemsResponseRowsItemBuilder {
        <PostV1ReportsCostCenterItemsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCostCenterItemsResponseRowsItemBuilder {
    cost_center_code: Option<String>,
    cost_center_name: Option<String>,
    item_name: Option<String>,
    net: Option<String>,
}

impl PostV1ReportsCostCenterItemsResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsCostCenterItemsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cost_center_code`](PostV1ReportsCostCenterItemsResponseRowsItemBuilder::cost_center_code)
    /// - [`cost_center_name`](PostV1ReportsCostCenterItemsResponseRowsItemBuilder::cost_center_name)
    /// - [`item_name`](PostV1ReportsCostCenterItemsResponseRowsItemBuilder::item_name)
    /// - [`net`](PostV1ReportsCostCenterItemsResponseRowsItemBuilder::net)
    pub fn build(self) -> Result<PostV1ReportsCostCenterItemsResponseRowsItem, BuildError> {
        Ok(PostV1ReportsCostCenterItemsResponseRowsItem {
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
