pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatchRulesListBankRequest {}

impl MatchRulesListBankRequest {
    pub fn builder() -> MatchRulesListBankRequestBuilder {
        <MatchRulesListBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatchRulesListBankRequestBuilder {}

impl MatchRulesListBankRequestBuilder {
    /// Consumes the builder and constructs a [`MatchRulesListBankRequest`].
    pub fn build(self) -> Result<MatchRulesListBankRequest, BuildError> {
        Ok(MatchRulesListBankRequest {})
    }
}
