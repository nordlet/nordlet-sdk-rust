pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub facts: PostV1DeclarationsDeReturnFactsSetRequestFacts,
}

impl PostV1DeclarationsDeReturnFactsSetRequest {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetRequestBuilder {
        <PostV1DeclarationsDeReturnFactsSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetRequestBuilder {
    year: Option<i64>,
    facts: Option<PostV1DeclarationsDeReturnFactsSetRequestFacts>,
}

impl PostV1DeclarationsDeReturnFactsSetRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn facts(mut self, value: PostV1DeclarationsDeReturnFactsSetRequestFacts) -> Self {
        self.facts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsDeReturnFactsSetRequestBuilder::year)
    /// - [`facts`](PostV1DeclarationsDeReturnFactsSetRequestBuilder::facts)
    pub fn build(self) -> Result<PostV1DeclarationsDeReturnFactsSetRequest, BuildError> {
        Ok(PostV1DeclarationsDeReturnFactsSetRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            facts: self
                .facts
                .ok_or_else(|| BuildError::missing_field("facts"))?,
        })
    }
}
