pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsModernizeResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemReason,
}

impl PostV1AssetsAssetsModernizeResponseInputVatUseChangesItem {
    pub fn builder() -> PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemBuilder {
        <PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemReason>,
}

impl PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemBuilder {
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
        value: PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsModernizeResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](PostV1AssetsAssetsModernizeResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(
        self,
    ) -> Result<PostV1AssetsAssetsModernizeResponseInputVatUseChangesItem, BuildError> {
        Ok(PostV1AssetsAssetsModernizeResponseInputVatUseChangesItem {
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
