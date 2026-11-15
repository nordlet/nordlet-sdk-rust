pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsCorrectionPreviewDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl SubmissionsCorrectionPreviewDeclarationsRequest {
    pub fn builder() -> SubmissionsCorrectionPreviewDeclarationsRequestBuilder {
        <SubmissionsCorrectionPreviewDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsCorrectionPreviewDeclarationsRequestBuilder {
    id: Option<String>,
}

impl SubmissionsCorrectionPreviewDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsCorrectionPreviewDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubmissionsCorrectionPreviewDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<SubmissionsCorrectionPreviewDeclarationsRequest, BuildError> {
        Ok(SubmissionsCorrectionPreviewDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
