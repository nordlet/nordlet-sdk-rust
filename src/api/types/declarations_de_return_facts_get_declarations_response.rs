pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub facts: DeReturnFactsGetDeclarationsResponseFacts,
}

impl DeReturnFactsGetDeclarationsResponse {
    pub fn builder() -> DeReturnFactsGetDeclarationsResponseBuilder {
        <DeReturnFactsGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsResponseBuilder {
    year: Option<i64>,
    facts: Option<DeReturnFactsGetDeclarationsResponseFacts>,
}

impl DeReturnFactsGetDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn facts(mut self, value: DeReturnFactsGetDeclarationsResponseFacts) -> Self {
        self.facts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeReturnFactsGetDeclarationsResponseBuilder::year)
    /// - [`facts`](DeReturnFactsGetDeclarationsResponseBuilder::facts)
    pub fn build(self) -> Result<DeReturnFactsGetDeclarationsResponse, BuildError> {
        Ok(DeReturnFactsGetDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            facts: self
                .facts
                .ok_or_else(|| BuildError::missing_field("facts"))?,
        })
    }
}
