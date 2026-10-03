pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub facts: PostV1DeclarationsDeReturnFactsSetResponseFacts,
}

impl PostV1DeclarationsDeReturnFactsSetResponse {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetResponseBuilder {
        <PostV1DeclarationsDeReturnFactsSetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetResponseBuilder {
    year: Option<i64>,
    facts: Option<PostV1DeclarationsDeReturnFactsSetResponseFacts>,
}

impl PostV1DeclarationsDeReturnFactsSetResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn facts(mut self, value: PostV1DeclarationsDeReturnFactsSetResponseFacts) -> Self {
        self.facts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsDeReturnFactsSetResponseBuilder::year)
    /// - [`facts`](PostV1DeclarationsDeReturnFactsSetResponseBuilder::facts)
    pub fn build(self) -> Result<PostV1DeclarationsDeReturnFactsSetResponse, BuildError> {
        Ok(PostV1DeclarationsDeReturnFactsSetResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            facts: self
                .facts
                .ok_or_else(|| BuildError::missing_field("facts"))?,
        })
    }
}
