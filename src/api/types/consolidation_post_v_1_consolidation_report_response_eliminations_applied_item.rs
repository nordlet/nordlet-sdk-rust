pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseEliminationsAppliedItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl PostV1ConsolidationReportResponseEliminationsAppliedItem {
    pub fn builder() -> PostV1ConsolidationReportResponseEliminationsAppliedItemBuilder {
        <PostV1ConsolidationReportResponseEliminationsAppliedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseEliminationsAppliedItemBuilder {
    code: Option<String>,
    amount: Option<String>,
    note: Option<String>,
}

impl PostV1ConsolidationReportResponseEliminationsAppliedItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseEliminationsAppliedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ConsolidationReportResponseEliminationsAppliedItemBuilder::code)
    /// - [`amount`](PostV1ConsolidationReportResponseEliminationsAppliedItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationReportResponseEliminationsAppliedItem, BuildError> {
        Ok(PostV1ConsolidationReportResponseEliminationsAppliedItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            note: self.note,
        })
    }
}
