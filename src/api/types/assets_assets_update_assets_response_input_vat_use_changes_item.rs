pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsUpdateAssetsResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: AssetsUpdateAssetsResponseInputVatUseChangesItemReason,
}

impl AssetsUpdateAssetsResponseInputVatUseChangesItem {
    pub fn builder() -> AssetsUpdateAssetsResponseInputVatUseChangesItemBuilder {
        <AssetsUpdateAssetsResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsUpdateAssetsResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<AssetsUpdateAssetsResponseInputVatUseChangesItemReason>,
}

impl AssetsUpdateAssetsResponseInputVatUseChangesItemBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    pub fn reason(mut self, value: AssetsUpdateAssetsResponseInputVatUseChangesItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsUpdateAssetsResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AssetsUpdateAssetsResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](AssetsUpdateAssetsResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](AssetsUpdateAssetsResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(self) -> Result<AssetsUpdateAssetsResponseInputVatUseChangesItem, BuildError> {
        Ok(AssetsUpdateAssetsResponseInputVatUseChangesItem {
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
