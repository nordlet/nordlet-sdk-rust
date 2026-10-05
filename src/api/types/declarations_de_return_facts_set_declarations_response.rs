pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub facts: DeReturnFactsSetDeclarationsResponseFacts,
}

impl DeReturnFactsSetDeclarationsResponse {
    pub fn builder() -> DeReturnFactsSetDeclarationsResponseBuilder {
        <DeReturnFactsSetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsResponseBuilder {
    year: Option<i64>,
    facts: Option<DeReturnFactsSetDeclarationsResponseFacts>,
}

impl DeReturnFactsSetDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn facts(mut self, value: DeReturnFactsSetDeclarationsResponseFacts) -> Self {
        self.facts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeReturnFactsSetDeclarationsResponseBuilder::year)
    /// - [`facts`](DeReturnFactsSetDeclarationsResponseBuilder::facts)
    pub fn build(self) -> Result<DeReturnFactsSetDeclarationsResponse, BuildError> {
        Ok(DeReturnFactsSetDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            facts: self
                .facts
                .ok_or_else(|| BuildError::missing_field("facts"))?,
        })
    }
}
