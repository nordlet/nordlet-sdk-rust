pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsCreateResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: PostV1AssetsAssetsCreateResponseInputVatUseChangesItemReason,
}

impl PostV1AssetsAssetsCreateResponseInputVatUseChangesItem {
    pub fn builder() -> PostV1AssetsAssetsCreateResponseInputVatUseChangesItemBuilder {
        <PostV1AssetsAssetsCreateResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsCreateResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<PostV1AssetsAssetsCreateResponseInputVatUseChangesItemReason>,
}

impl PostV1AssetsAssetsCreateResponseInputVatUseChangesItemBuilder {
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
        value: PostV1AssetsAssetsCreateResponseInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsCreateResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsAssetsCreateResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](PostV1AssetsAssetsCreateResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](PostV1AssetsAssetsCreateResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(
        self,
    ) -> Result<PostV1AssetsAssetsCreateResponseInputVatUseChangesItem, BuildError> {
        Ok(PostV1AssetsAssetsCreateResponseInputVatUseChangesItem {
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
