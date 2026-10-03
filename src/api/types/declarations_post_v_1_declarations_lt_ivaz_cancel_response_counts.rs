pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazCancelResponseCounts {
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsLtIvazCancelResponseCounts {
    pub fn builder() -> PostV1DeclarationsLtIvazCancelResponseCountsBuilder {
        <PostV1DeclarationsLtIvazCancelResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazCancelResponseCountsBuilder {
    documents: Option<i64>,
}

impl PostV1DeclarationsLtIvazCancelResponseCountsBuilder {
    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazCancelResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](PostV1DeclarationsLtIvazCancelResponseCountsBuilder::documents)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazCancelResponseCounts, BuildError> {
        Ok(PostV1DeclarationsLtIvazCancelResponseCounts {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
