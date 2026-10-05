pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatchRulesDeleteBankResponse {
    #[serde(default)]
    pub id: String,
}

impl MatchRulesDeleteBankResponse {
    pub fn builder() -> MatchRulesDeleteBankResponseBuilder {
        <MatchRulesDeleteBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatchRulesDeleteBankResponseBuilder {
    id: Option<String>,
}

impl MatchRulesDeleteBankResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MatchRulesDeleteBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MatchRulesDeleteBankResponseBuilder::id)
    pub fn build(self) -> Result<MatchRulesDeleteBankResponse, BuildError> {
        Ok(MatchRulesDeleteBankResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
