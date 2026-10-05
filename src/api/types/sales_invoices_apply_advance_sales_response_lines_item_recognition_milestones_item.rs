pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItem {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<NaiveDate>,
    #[serde(default)]
    pub percent: String,
}

impl InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItem {
    pub fn builder() -> InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItemBuilder {
        <InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItemBuilder {
    description: Option<String>,
    expected_date: Option<NaiveDate>,
    percent: Option<String>,
}

impl InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn expected_date(mut self, value: NaiveDate) -> Self {
        self.expected_date = Some(value);
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItemBuilder::description)
    /// - [`percent`](InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItemBuilder::percent)
    pub fn build(
        self,
    ) -> Result<InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItem, BuildError>
    {
        Ok(
            InvoicesApplyAdvanceSalesResponseLinesItemRecognitionMilestonesItem {
                description: self
                    .description
                    .ok_or_else(|| BuildError::missing_field("description"))?,
                expected_date: self.expected_date,
                percent: self
                    .percent
                    .ok_or_else(|| BuildError::missing_field("percent"))?,
            },
        )
    }
}
