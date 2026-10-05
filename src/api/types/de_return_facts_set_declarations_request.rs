pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub facts: DeReturnFactsSetDeclarationsRequestFacts,
}

impl DeReturnFactsSetDeclarationsRequest {
    pub fn builder() -> DeReturnFactsSetDeclarationsRequestBuilder {
        <DeReturnFactsSetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsRequestBuilder {
    year: Option<i64>,
    facts: Option<DeReturnFactsSetDeclarationsRequestFacts>,
}

impl DeReturnFactsSetDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn facts(mut self, value: DeReturnFactsSetDeclarationsRequestFacts) -> Self {
        self.facts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeReturnFactsSetDeclarationsRequestBuilder::year)
    /// - [`facts`](DeReturnFactsSetDeclarationsRequestBuilder::facts)
    pub fn build(self) -> Result<DeReturnFactsSetDeclarationsRequest, BuildError> {
        Ok(DeReturnFactsSetDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            facts: self
                .facts
                .ok_or_else(|| BuildError::missing_field("facts"))?,
        })
    }
}
