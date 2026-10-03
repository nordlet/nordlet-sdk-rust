pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsGetResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub facts: PostV1DeclarationsDeReturnFactsGetResponseFacts,
}

impl PostV1DeclarationsDeReturnFactsGetResponse {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsGetResponseBuilder {
        <PostV1DeclarationsDeReturnFactsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsGetResponseBuilder {
    year: Option<i64>,
    facts: Option<PostV1DeclarationsDeReturnFactsGetResponseFacts>,
}

impl PostV1DeclarationsDeReturnFactsGetResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn facts(mut self, value: PostV1DeclarationsDeReturnFactsGetResponseFacts) -> Self {
        self.facts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsDeReturnFactsGetResponseBuilder::year)
    /// - [`facts`](PostV1DeclarationsDeReturnFactsGetResponseBuilder::facts)
    pub fn build(self) -> Result<PostV1DeclarationsDeReturnFactsGetResponse, BuildError> {
        Ok(PostV1DeclarationsDeReturnFactsGetResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            facts: self
                .facts
                .ok_or_else(|| BuildError::missing_field("facts"))?,
        })
    }
}
