pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsInputVatAssetsRequestInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: AssetsInputVatAssetsRequestInputVatUseChangesItemReason,
}

impl AssetsInputVatAssetsRequestInputVatUseChangesItem {
    pub fn builder() -> AssetsInputVatAssetsRequestInputVatUseChangesItemBuilder {
        <AssetsInputVatAssetsRequestInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsInputVatAssetsRequestInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<AssetsInputVatAssetsRequestInputVatUseChangesItemReason>,
}

impl AssetsInputVatAssetsRequestInputVatUseChangesItemBuilder {
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
        value: AssetsInputVatAssetsRequestInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsInputVatAssetsRequestInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AssetsInputVatAssetsRequestInputVatUseChangesItemBuilder::year)
    /// - [`percent`](AssetsInputVatAssetsRequestInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](AssetsInputVatAssetsRequestInputVatUseChangesItemBuilder::reason)
    pub fn build(self) -> Result<AssetsInputVatAssetsRequestInputVatUseChangesItem, BuildError> {
        Ok(AssetsInputVatAssetsRequestInputVatUseChangesItem {
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
