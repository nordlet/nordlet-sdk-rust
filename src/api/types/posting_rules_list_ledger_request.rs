pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostingRulesListLedgerRequest {}

impl PostingRulesListLedgerRequest {
    pub fn builder() -> PostingRulesListLedgerRequestBuilder {
        <PostingRulesListLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostingRulesListLedgerRequestBuilder {}

impl PostingRulesListLedgerRequestBuilder {
    /// Consumes the builder and constructs a [`PostingRulesListLedgerRequest`].
    pub fn build(self) -> Result<PostingRulesListLedgerRequest, BuildError> {
        Ok(PostingRulesListLedgerRequest {})
    }
}
