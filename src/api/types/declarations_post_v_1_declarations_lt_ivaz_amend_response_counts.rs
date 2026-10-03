pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazAmendResponseCounts {
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsLtIvazAmendResponseCounts {
    pub fn builder() -> PostV1DeclarationsLtIvazAmendResponseCountsBuilder {
        <PostV1DeclarationsLtIvazAmendResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazAmendResponseCountsBuilder {
    documents: Option<i64>,
}

impl PostV1DeclarationsLtIvazAmendResponseCountsBuilder {
    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazAmendResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](PostV1DeclarationsLtIvazAmendResponseCountsBuilder::documents)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazAmendResponseCounts, BuildError> {
        Ok(PostV1DeclarationsLtIvazAmendResponseCounts {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
