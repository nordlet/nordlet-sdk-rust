pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsListAssetsResponseRowsItemInputVatUseChangesItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub percent: String,
    pub reason: AssetsListAssetsResponseRowsItemInputVatUseChangesItemReason,
}

impl AssetsListAssetsResponseRowsItemInputVatUseChangesItem {
    pub fn builder() -> AssetsListAssetsResponseRowsItemInputVatUseChangesItemBuilder {
        <AssetsListAssetsResponseRowsItemInputVatUseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsListAssetsResponseRowsItemInputVatUseChangesItemBuilder {
    year: Option<i64>,
    percent: Option<String>,
    reason: Option<AssetsListAssetsResponseRowsItemInputVatUseChangesItemReason>,
}

impl AssetsListAssetsResponseRowsItemInputVatUseChangesItemBuilder {
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
        value: AssetsListAssetsResponseRowsItemInputVatUseChangesItemReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsListAssetsResponseRowsItemInputVatUseChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AssetsListAssetsResponseRowsItemInputVatUseChangesItemBuilder::year)
    /// - [`percent`](AssetsListAssetsResponseRowsItemInputVatUseChangesItemBuilder::percent)
    /// - [`reason`](AssetsListAssetsResponseRowsItemInputVatUseChangesItemBuilder::reason)
    pub fn build(
        self,
    ) -> Result<AssetsListAssetsResponseRowsItemInputVatUseChangesItem, BuildError> {
        Ok(AssetsListAssetsResponseRowsItemInputVatUseChangesItem {
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
