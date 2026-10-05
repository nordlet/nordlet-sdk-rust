pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsModernizeAssetsResponseInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: AssetsModernizeAssetsResponseInputVatUseChangesItemReason,
}

impl AssetsModernizeAssetsResponseInputVatUseChangesItem {
    pub fn builder() -> AssetsModernizeAssetsResponseInputVatUseChangesItemBuilder {
        <AssetsModernizeAssetsResponseInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsModernizeAssetsResponseInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<AssetsModernizeAssetsResponseInputVatUseChangesItemReason>,
}

impl AssetsModernizeAssetsResponseInputVatUseChangesItemBuilder {
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
        value: AssetsModernizeAssetsResponseInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsModernizeAssetsResponseInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AssetsModernizeAssetsResponseInputVatUseChangesItemBuilder::year)
    /// - [`percent`](AssetsModernizeAssetsResponseInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](AssetsModernizeAssetsResponseInputVatUseChangesItemBuilder::reason)
    pub fn build(self) -> Result<AssetsModernizeAssetsResponseInputVatUseChangesItem, BuildError> {
        Ok(AssetsModernizeAssetsResponseInputVatUseChangesItem {
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
