pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<NaiveDate>,
    #[serde(default)]
    pub percent: String,
}

impl InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem {
    pub fn builder() -> InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItemBuilder {
        <InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItemBuilder {
    description: Option<String>,
    expected_date: Option<NaiveDate>,
    percent: Option<String>,
}

impl InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItemBuilder {
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

    /// Consumes the builder and constructs a [`InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItemBuilder::description)
    /// - [`percent`](InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItemBuilder::percent)
    pub fn build(
        self,
    ) -> Result<InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem, BuildError> {
        Ok(
            InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem {
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
