pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazGenerateResponseCounts {
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsLtIvazGenerateResponseCounts {
    pub fn builder() -> PostV1DeclarationsLtIvazGenerateResponseCountsBuilder {
        <PostV1DeclarationsLtIvazGenerateResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazGenerateResponseCountsBuilder {
    documents: Option<i64>,
}

impl PostV1DeclarationsLtIvazGenerateResponseCountsBuilder {
    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazGenerateResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](PostV1DeclarationsLtIvazGenerateResponseCountsBuilder::documents)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazGenerateResponseCounts, BuildError> {
        Ok(PostV1DeclarationsLtIvazGenerateResponseCounts {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
