pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatchRulesListBankResponse {
    #[serde(default)]
    pub rows: Vec<MatchRulesListBankResponseRowsItem>,
}

impl MatchRulesListBankResponse {
    pub fn builder() -> MatchRulesListBankResponseBuilder {
        <MatchRulesListBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatchRulesListBankResponseBuilder {
    rows: Option<Vec<MatchRulesListBankResponseRowsItem>>,
}

impl MatchRulesListBankResponseBuilder {
    pub fn rows(mut self, value: Vec<MatchRulesListBankResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatchRulesListBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](MatchRulesListBankResponseBuilder::rows)
    pub fn build(self) -> Result<MatchRulesListBankResponse, BuildError> {
        Ok(MatchRulesListBankResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
