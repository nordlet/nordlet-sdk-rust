pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItem {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<NaiveDate>,
    #[serde(default)]
    pub percent: String,
}

impl InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItem {
    pub fn builder() -> InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItemBuilder {
        <InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItemBuilder {
    description: Option<String>,
    expected_date: Option<NaiveDate>,
    percent: Option<String>,
}

impl InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItemBuilder {
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

    /// Consumes the builder and constructs a [`InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItemBuilder::description)
    /// - [`percent`](InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItemBuilder::percent)
    pub fn build(
        self,
    ) -> Result<InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItem, BuildError> {
        Ok(
            InvoicesReverseSalesResponseLinesItemRecognitionMilestonesItem {
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
