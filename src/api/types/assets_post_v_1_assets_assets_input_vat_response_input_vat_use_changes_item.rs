pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemReason,
}

impl PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem {
    pub fn builder() -> PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemBuilder {
        <PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemReason>,
}

impl PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemBuilder {
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
        value: PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](PostV1AssetsAssetsInputVatResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(
        self,
    ) -> Result<PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem, BuildError> {
        Ok(PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem {
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
