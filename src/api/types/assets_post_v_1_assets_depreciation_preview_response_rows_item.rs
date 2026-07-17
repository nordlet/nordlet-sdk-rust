pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsDepreciationPreviewResponseRowsItem {
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
}

impl PostV1AssetsDepreciationPreviewResponseRowsItem {
    pub fn builder() -> PostV1AssetsDepreciationPreviewResponseRowsItemBuilder {
        <PostV1AssetsDepreciationPreviewResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsDepreciationPreviewResponseRowsItemBuilder {
    asset_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    amount: Option<String>,
    already_posted: Option<bool>,
}

impl PostV1AssetsDepreciationPreviewResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AssetsDepreciationPreviewResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](PostV1AssetsDepreciationPreviewResponseRowsItemBuilder::asset_id)
    /// - [`code`](PostV1AssetsDepreciationPreviewResponseRowsItemBuilder::code)
    /// - [`name`](PostV1AssetsDepreciationPreviewResponseRowsItemBuilder::name)
    /// - [`amount`](PostV1AssetsDepreciationPreviewResponseRowsItemBuilder::amount)
    /// - [`already_posted`](PostV1AssetsDepreciationPreviewResponseRowsItemBuilder::already_posted)
    pub fn build(self) -> Result<PostV1AssetsDepreciationPreviewResponseRowsItem, BuildError> {
        Ok(PostV1AssetsDepreciationPreviewResponseRowsItem {
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
        })
    }
}
