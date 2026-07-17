pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseTrialBalanceItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub closing: String,
    #[serde(default)]
    pub period: String,
}

impl PostV1ConsolidationReportResponseTrialBalanceItem {
    pub fn builder() -> PostV1ConsolidationReportResponseTrialBalanceItemBuilder {
        <PostV1ConsolidationReportResponseTrialBalanceItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseTrialBalanceItemBuilder {
    code: Option<String>,
    r#type: Option<String>,
    closing: Option<String>,
    period: Option<String>,
}

impl PostV1ConsolidationReportResponseTrialBalanceItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseTrialBalanceItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ConsolidationReportResponseTrialBalanceItemBuilder::code)
    /// - [`r#type`](PostV1ConsolidationReportResponseTrialBalanceItemBuilder::r#type)
    /// - [`closing`](PostV1ConsolidationReportResponseTrialBalanceItemBuilder::closing)
    /// - [`period`](PostV1ConsolidationReportResponseTrialBalanceItemBuilder::period)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponseTrialBalanceItem, BuildError> {
        Ok(PostV1ConsolidationReportResponseTrialBalanceItem {
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
