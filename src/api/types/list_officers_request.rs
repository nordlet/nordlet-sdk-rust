pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOfficersRequest {}

impl ListOfficersRequest {
    pub fn builder() -> ListOfficersRequestBuilder {
        <ListOfficersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOfficersRequestBuilder {}

impl ListOfficersRequestBuilder {
    /// Consumes the builder and constructs a [`ListOfficersRequest`].
    pub fn build(self) -> Result<ListOfficersRequest, BuildError> {
        Ok(ListOfficersRequest {})
    }
}
