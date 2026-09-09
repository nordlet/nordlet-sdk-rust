pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMatchRulesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1BankMatchRulesListResponseRowsItem>,
}

impl PostV1BankMatchRulesListResponse {
    pub fn builder() -> PostV1BankMatchRulesListResponseBuilder {
        <PostV1BankMatchRulesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMatchRulesListResponseBuilder {
    rows: Option<Vec<PostV1BankMatchRulesListResponseRowsItem>>,
}

impl PostV1BankMatchRulesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1BankMatchRulesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMatchRulesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1BankMatchRulesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1BankMatchRulesListResponse, BuildError> {
        Ok(PostV1BankMatchRulesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
