pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatusesListPartnersRequest {}

impl StatusesListPartnersRequest {
    pub fn builder() -> StatusesListPartnersRequestBuilder {
        <StatusesListPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusesListPartnersRequestBuilder {}

impl StatusesListPartnersRequestBuilder {
    /// Consumes the builder and constructs a [`StatusesListPartnersRequest`].
    pub fn build(self) -> Result<StatusesListPartnersRequest, BuildError> {
        Ok(StatusesListPartnersRequest {})
    }
}
