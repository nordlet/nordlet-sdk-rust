pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemReason,
}

impl PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem {
    pub fn builder() -> PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemBuilder {
        <PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemReason>,
}

impl PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemBuilder {
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
        value: PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemBuilder::year)
    /// - [`percent`](PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemBuilder::reason)
    pub fn build(
        self,
    ) -> Result<PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem, BuildError> {
        Ok(PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem {
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
