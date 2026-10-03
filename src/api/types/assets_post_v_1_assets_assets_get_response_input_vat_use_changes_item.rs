pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsGetResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: PostV1AssetsAssetsGetResponseInputVatUseChangesItemReason,
}

impl PostV1AssetsAssetsGetResponseInputVatUseChangesItem {
    pub fn builder() -> PostV1AssetsAssetsGetResponseInputVatUseChangesItemBuilder {
        <PostV1AssetsAssetsGetResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsGetResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<PostV1AssetsAssetsGetResponseInputVatUseChangesItemReason>,
}

impl PostV1AssetsAssetsGetResponseInputVatUseChangesItemBuilder {
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
        value: PostV1AssetsAssetsGetResponseInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsGetResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsAssetsGetResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](PostV1AssetsAssetsGetResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](PostV1AssetsAssetsGetResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(self) -> Result<PostV1AssetsAssetsGetResponseInputVatUseChangesItem, BuildError> {
        Ok(PostV1AssetsAssetsGetResponseInputVatUseChangesItem {
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
