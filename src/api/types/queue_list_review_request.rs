pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueueListReviewRequest {}

impl QueueListReviewRequest {
    pub fn builder() -> QueueListReviewRequestBuilder {
        <QueueListReviewRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueueListReviewRequestBuilder {}

impl QueueListReviewRequestBuilder {
    /// Consumes the builder and constructs a [`QueueListReviewRequest`].
    pub fn build(self) -> Result<QueueListReviewRequest, BuildError> {
        Ok(QueueListReviewRequest {})
    }
}
