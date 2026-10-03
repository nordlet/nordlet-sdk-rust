pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsSchemesRequest {}

impl PostV1LedgerStatementRowsSchemesRequest {
    pub fn builder() -> PostV1LedgerStatementRowsSchemesRequestBuilder {
        <PostV1LedgerStatementRowsSchemesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsSchemesRequestBuilder {}

impl PostV1LedgerStatementRowsSchemesRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsSchemesRequest`].
    pub fn build(self) -> Result<PostV1LedgerStatementRowsSchemesRequest, BuildError> {
        Ok(PostV1LedgerStatementRowsSchemesRequest {})
    }
}
