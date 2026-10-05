pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsCreateAssetsResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: AssetsCreateAssetsResponseInputVatUseChangesItemReason,
}

impl AssetsCreateAssetsResponseInputVatUseChangesItem {
    pub fn builder() -> AssetsCreateAssetsResponseInputVatUseChangesItemBuilder {
        <AssetsCreateAssetsResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsCreateAssetsResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<AssetsCreateAssetsResponseInputVatUseChangesItemReason>,
}

impl AssetsCreateAssetsResponseInputVatUseChangesItemBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    pub fn reason(mut self, value: AssetsCreateAssetsResponseInputVatUseChangesItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsCreateAssetsResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AssetsCreateAssetsResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](AssetsCreateAssetsResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](AssetsCreateAssetsResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(self) -> Result<AssetsCreateAssetsResponseInputVatUseChangesItem, BuildError> {
        Ok(AssetsCreateAssetsResponseInputVatUseChangesItem {
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
