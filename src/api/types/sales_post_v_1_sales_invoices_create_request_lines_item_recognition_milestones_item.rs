pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItem {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<String>,
    #[serde(default)]
    pub percent: String,
}

impl PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItem {
    pub fn builder() -> PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItemBuilder {
        <PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItemBuilder {
    description: Option<String>,
    expected_date: Option<String>,
    percent: Option<String>,
}

impl PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItemBuilder::description)
    /// - [`percent`](PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItemBuilder::percent)
    pub fn build(
        self,
    ) -> Result<PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItem, BuildError>
    {
        Ok(
            PostV1SalesInvoicesCreateRequestLinesItemRecognitionMilestonesItem {
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
