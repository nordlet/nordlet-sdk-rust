pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsCorrectionPreviewDeclarationsResponse {
    #[serde(default)]
    pub needed: bool,
    #[serde(rename = "changeCount")]
    #[serde(default)]
    pub change_count: i64,
    #[serde(default)]
    pub changes: Vec<SubmissionsCorrectionPreviewDeclarationsResponseChangesItem>,
}

impl SubmissionsCorrectionPreviewDeclarationsResponse {
    pub fn builder() -> SubmissionsCorrectionPreviewDeclarationsResponseBuilder {
        <SubmissionsCorrectionPreviewDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsCorrectionPreviewDeclarationsResponseBuilder {
    needed: Option<bool>,
    change_count: Option<i64>,
    changes: Option<Vec<SubmissionsCorrectionPreviewDeclarationsResponseChangesItem>>,
}

impl SubmissionsCorrectionPreviewDeclarationsResponseBuilder {
    pub fn needed(mut self, value: bool) -> Self {
        self.needed = Some(value);
        self
    }

    pub fn change_count(mut self, value: i64) -> Self {
        self.change_count = Some(value);
        self
    }

    pub fn changes(
        mut self,
        value: Vec<SubmissionsCorrectionPreviewDeclarationsResponseChangesItem>,
    ) -> Self {
        self.changes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsCorrectionPreviewDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`needed`](SubmissionsCorrectionPreviewDeclarationsResponseBuilder::needed)
    /// - [`change_count`](SubmissionsCorrectionPreviewDeclarationsResponseBuilder::change_count)
    /// - [`changes`](SubmissionsCorrectionPreviewDeclarationsResponseBuilder::changes)
    pub fn build(self) -> Result<SubmissionsCorrectionPreviewDeclarationsResponse, BuildError> {
        Ok(SubmissionsCorrectionPreviewDeclarationsResponse {
            needed: self
                .needed
                .ok_or_else(|| BuildError::missing_field("needed"))?,
            change_count: self
                .change_count
                .ok_or_else(|| BuildError::missing_field("change_count"))?,
            changes: self
                .changes
                .ok_or_else(|| BuildError::missing_field("changes"))?,
        })
    }
}
