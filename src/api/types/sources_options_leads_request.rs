pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesOptionsLeadsRequest {}

impl SourcesOptionsLeadsRequest {
    pub fn builder() -> SourcesOptionsLeadsRequestBuilder {
        <SourcesOptionsLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesOptionsLeadsRequestBuilder {}

impl SourcesOptionsLeadsRequestBuilder {
    /// Consumes the builder and constructs a [`SourcesOptionsLeadsRequest`].
    pub fn build(self) -> Result<SourcesOptionsLeadsRequest, BuildError> {
        Ok(SourcesOptionsLeadsRequest {})
    }
}
