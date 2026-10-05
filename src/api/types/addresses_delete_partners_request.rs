pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddressesDeletePartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl AddressesDeletePartnersRequest {
    pub fn builder() -> AddressesDeletePartnersRequestBuilder {
        <AddressesDeletePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddressesDeletePartnersRequestBuilder {
    id: Option<String>,
}

impl AddressesDeletePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddressesDeletePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AddressesDeletePartnersRequestBuilder::id)
    pub fn build(self) -> Result<AddressesDeletePartnersRequest, BuildError> {
        Ok(AddressesDeletePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
