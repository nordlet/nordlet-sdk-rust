pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueueListReviewResponse {
    #[serde(default)]
    pub items: Vec<QueueListReviewResponseItemsItem>,
    #[serde(default)]
    pub total: i64,
}

impl QueueListReviewResponse {
    pub fn builder() -> QueueListReviewResponseBuilder {
        <QueueListReviewResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueueListReviewResponseBuilder {
    items: Option<Vec<QueueListReviewResponseItemsItem>>,
    total: Option<i64>,
}

impl QueueListReviewResponseBuilder {
    pub fn items(mut self, value: Vec<QueueListReviewResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueueListReviewResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](QueueListReviewResponseBuilder::items)
    /// - [`total`](QueueListReviewResponseBuilder::total)
    pub fn build(self) -> Result<QueueListReviewResponse, BuildError> {
        Ok(QueueListReviewResponse {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
