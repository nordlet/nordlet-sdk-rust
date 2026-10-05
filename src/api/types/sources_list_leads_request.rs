pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesListLeadsRequest {}

impl SourcesListLeadsRequest {
    pub fn builder() -> SourcesListLeadsRequestBuilder {
        <SourcesListLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesListLeadsRequestBuilder {}

impl SourcesListLeadsRequestBuilder {
    /// Consumes the builder and constructs a [`SourcesListLeadsRequest`].
    pub fn build(self) -> Result<SourcesListLeadsRequest, BuildError> {
        Ok(SourcesListLeadsRequest {})
    }
}
