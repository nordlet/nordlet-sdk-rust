pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsDisposeAssetsResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: AssetsDisposeAssetsResponseInputVatUseChangesItemReason,
}

impl AssetsDisposeAssetsResponseInputVatUseChangesItem {
    pub fn builder() -> AssetsDisposeAssetsResponseInputVatUseChangesItemBuilder {
        <AssetsDisposeAssetsResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsDisposeAssetsResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<AssetsDisposeAssetsResponseInputVatUseChangesItemReason>,
}

impl AssetsDisposeAssetsResponseInputVatUseChangesItemBuilder {
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
        value: AssetsDisposeAssetsResponseInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsDisposeAssetsResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AssetsDisposeAssetsResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](AssetsDisposeAssetsResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](AssetsDisposeAssetsResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(self) -> Result<AssetsDisposeAssetsResponseInputVatUseChangesItem, BuildError> {
        Ok(AssetsDisposeAssetsResponseInputVatUseChangesItem {
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
