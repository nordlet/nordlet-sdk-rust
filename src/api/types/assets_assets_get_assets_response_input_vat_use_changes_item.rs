pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsGetAssetsResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: AssetsGetAssetsResponseInputVatUseChangesItemReason,
}

impl AssetsGetAssetsResponseInputVatUseChangesItem {
    pub fn builder() -> AssetsGetAssetsResponseInputVatUseChangesItemBuilder {
        <AssetsGetAssetsResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsGetAssetsResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<AssetsGetAssetsResponseInputVatUseChangesItemReason>,
}

impl AssetsGetAssetsResponseInputVatUseChangesItemBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    pub fn reason(mut self, value: AssetsGetAssetsResponseInputVatUseChangesItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsGetAssetsResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AssetsGetAssetsResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](AssetsGetAssetsResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](AssetsGetAssetsResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(self) -> Result<AssetsGetAssetsResponseInputVatUseChangesItem, BuildError> {
        Ok(AssetsGetAssetsResponseInputVatUseChangesItem {
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
