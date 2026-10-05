pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseEliminationsAppliedItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl ReportConsolidationResponseEliminationsAppliedItem {
    pub fn builder() -> ReportConsolidationResponseEliminationsAppliedItemBuilder {
        <ReportConsolidationResponseEliminationsAppliedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseEliminationsAppliedItemBuilder {
    code: Option<String>,
    amount: Option<String>,
    note: Option<String>,
}

impl ReportConsolidationResponseEliminationsAppliedItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseEliminationsAppliedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ReportConsolidationResponseEliminationsAppliedItemBuilder::code)
    /// - [`amount`](ReportConsolidationResponseEliminationsAppliedItemBuilder::amount)
    pub fn build(self) -> Result<ReportConsolidationResponseEliminationsAppliedItem, BuildError> {
        Ok(ReportConsolidationResponseEliminationsAppliedItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            note: self.note,
        })
    }
}
