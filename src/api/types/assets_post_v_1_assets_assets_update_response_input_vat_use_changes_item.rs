pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsUpdateResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemReason,
}

impl PostV1AssetsAssetsUpdateResponseInputVatUseChangesItem {
    pub fn builder() -> PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemBuilder {
        <PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemReason>,
}

impl PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemBuilder {
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
        value: PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsUpdateResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](PostV1AssetsAssetsUpdateResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(
        self,
    ) -> Result<PostV1AssetsAssetsUpdateResponseInputVatUseChangesItem, BuildError> {
        Ok(PostV1AssetsAssetsUpdateResponseInputVatUseChangesItem {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            percent: self
                .percent
                .ok_or_else(|| BuildError::missing_field("percent"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
