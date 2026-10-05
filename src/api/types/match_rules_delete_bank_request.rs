pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatchRulesDeleteBankRequest {
    #[serde(default)]
    pub id: String,
}

impl MatchRulesDeleteBankRequest {
    pub fn builder() -> MatchRulesDeleteBankRequestBuilder {
        <MatchRulesDeleteBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatchRulesDeleteBankRequestBuilder {
    id: Option<String>,
}

impl MatchRulesDeleteBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MatchRulesDeleteBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MatchRulesDeleteBankRequestBuilder::id)
    pub fn build(self) -> Result<MatchRulesDeleteBankRequest, BuildError> {
        Ok(MatchRulesDeleteBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
