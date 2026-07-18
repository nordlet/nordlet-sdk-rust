pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesUpdateRequestLinesItemRecognition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PostV1SalesInvoicesUpdateRequestLinesItemRecognitionMethod>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<PostV1SalesInvoicesUpdateRequestLinesItemRecognitionMilestonesItem>>,
}

impl PostV1SalesInvoicesUpdateRequestLinesItemRecognition {
    pub fn builder() -> PostV1SalesInvoicesUpdateRequestLinesItemRecognitionBuilder {
        <PostV1SalesInvoicesUpdateRequestLinesItemRecognitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUpdateRequestLinesItemRecognitionBuilder {
    method: Option<PostV1SalesInvoicesUpdateRequestLinesItemRecognitionMethod>,
    start_date: Option<String>,
    end_date: Option<String>,
    milestones: Option<Vec<PostV1SalesInvoicesUpdateRequestLinesItemRecognitionMilestonesItem>>,
}

impl PostV1SalesInvoicesUpdateRequestLinesItemRecognitionBuilder {
    pub fn method(
        mut self,
        value: PostV1SalesInvoicesUpdateRequestLinesItemRecognitionMethod,
    ) -> Self {
        self.method = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn milestones(
        mut self,
        value: Vec<PostV1SalesInvoicesUpdateRequestLinesItemRecognitionMilestonesItem>,
    ) -> Self {
        self.milestones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUpdateRequestLinesItemRecognition`].
    pub fn build(self) -> Result<PostV1SalesInvoicesUpdateRequestLinesItemRecognition, BuildError> {
        Ok(PostV1SalesInvoicesUpdateRequestLinesItemRecognition {
            method: self.method,
            start_date: self.start_date,
            end_date: self.end_date,
            milestones: self.milestones,
        })
    }
}
