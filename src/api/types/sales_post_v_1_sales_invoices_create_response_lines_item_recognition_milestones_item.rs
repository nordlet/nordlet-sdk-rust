pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItem {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<String>,
    #[serde(default)]
    pub percent: String,
}

impl PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItem {
    pub fn builder() -> PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItemBuilder {
        <PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItemBuilder {
    description: Option<String>,
    expected_date: Option<String>,
    percent: Option<String>,
}

impl PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItemBuilder::description)
    /// - [`percent`](PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItemBuilder::percent)
    pub fn build(
        self,
    ) -> Result<PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItem, BuildError>
    {
        Ok(
            PostV1SalesInvoicesCreateResponseLinesItemRecognitionMilestonesItem {
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
