pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItem {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<String>,
    #[serde(default)]
    pub percent: String,
}

impl PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItem {
    pub fn builder() -> PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItemBuilder {
        <PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItemBuilder {
    description: Option<String>,
    expected_date: Option<String>,
    percent: Option<String>,
}

impl PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn expected_date(mut self, value: impl Into<String>) -> Self {
        self.expected_date = Some(value.into());
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItemBuilder::description)
    /// - [`percent`](PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItemBuilder::percent)
    pub fn build(
        self,
    ) -> Result<PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItem, BuildError>
    {
        Ok(
            PostV1SalesInvoicesIssueResponseLinesItemRecognitionMilestonesItem {
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
