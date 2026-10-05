pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseTrialBalanceItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub closing: String,
    #[serde(default)]
    pub period: String,
}

impl ReportConsolidationResponseTrialBalanceItem {
    pub fn builder() -> ReportConsolidationResponseTrialBalanceItemBuilder {
        <ReportConsolidationResponseTrialBalanceItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseTrialBalanceItemBuilder {
    code: Option<String>,
    r#type: Option<String>,
    closing: Option<String>,
    period: Option<String>,
}

impl ReportConsolidationResponseTrialBalanceItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn closing(mut self, value: impl Into<String>) -> Self {
        self.closing = Some(value.into());
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseTrialBalanceItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ReportConsolidationResponseTrialBalanceItemBuilder::code)
    /// - [`r#type`](ReportConsolidationResponseTrialBalanceItemBuilder::r#type)
    /// - [`closing`](ReportConsolidationResponseTrialBalanceItemBuilder::closing)
    /// - [`period`](ReportConsolidationResponseTrialBalanceItemBuilder::period)
    pub fn build(self) -> Result<ReportConsolidationResponseTrialBalanceItem, BuildError> {
        Ok(ReportConsolidationResponseTrialBalanceItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            closing: self
                .closing
                .ok_or_else(|| BuildError::missing_field("closing"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
