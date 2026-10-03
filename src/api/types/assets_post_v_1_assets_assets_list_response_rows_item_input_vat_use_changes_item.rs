pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemReason,
}

impl PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItem {
    pub fn builder() -> PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemBuilder {
        <PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemReason>,
}

impl PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    pub fn reason(
        mut self,
        value: PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemBuilder::year)
    /// - [`percent`](PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItemBuilder::reason)
    pub fn build(
        self,
    ) -> Result<PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItem, BuildError> {
        Ok(
            PostV1AssetsAssetsListResponseRowsItemInputVatUseChangesItem {
                year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
                percent: self
                    .percent
                    .ok_or_else(|| BuildError::missing_field("percent"))?,
                reason: self
                    .reason
                    .ok_or_else(|| BuildError::missing_field("reason"))?,
            },
        )
    }
}
