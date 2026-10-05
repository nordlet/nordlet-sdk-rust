pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepreciationPreviewAssetsResponseRowsItem {
    #[serde(rename = "assetId")]
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "alreadyPosted")]
    #[serde(default)]
    pub already_posted: bool,
    /// Every month this run posts for the asset, earlier unposted months included
    #[serde(default)]
    pub months: Vec<String>,
}

impl DepreciationPreviewAssetsResponseRowsItem {
    pub fn builder() -> DepreciationPreviewAssetsResponseRowsItemBuilder {
        <DepreciationPreviewAssetsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepreciationPreviewAssetsResponseRowsItemBuilder {
    asset_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    amount: Option<String>,
    already_posted: Option<bool>,
    months: Option<Vec<String>>,
}

impl DepreciationPreviewAssetsResponseRowsItemBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn already_posted(mut self, value: bool) -> Self {
        self.already_posted = Some(value);
        self
    }

    pub fn months(mut self, value: Vec<String>) -> Self {
        self.months = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DepreciationPreviewAssetsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DepreciationPreviewAssetsResponseRowsItemBuilder::asset_id)
    /// - [`code`](DepreciationPreviewAssetsResponseRowsItemBuilder::code)
    /// - [`name`](DepreciationPreviewAssetsResponseRowsItemBuilder::name)
    /// - [`amount`](DepreciationPreviewAssetsResponseRowsItemBuilder::amount)
    /// - [`already_posted`](DepreciationPreviewAssetsResponseRowsItemBuilder::already_posted)
    /// - [`months`](DepreciationPreviewAssetsResponseRowsItemBuilder::months)
    pub fn build(self) -> Result<DepreciationPreviewAssetsResponseRowsItem, BuildError> {
        Ok(DepreciationPreviewAssetsResponseRowsItem {
            asset_id: self
                .asset_id
                .ok_or_else(|| BuildError::missing_field("asset_id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            already_posted: self
                .already_posted
                .ok_or_else(|| BuildError::missing_field("already_posted"))?,
            months: self
                .months
                .ok_or_else(|| BuildError::missing_field("months"))?,
        })
    }
}
