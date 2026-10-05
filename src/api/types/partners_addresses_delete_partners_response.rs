pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddressesDeletePartnersResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl AddressesDeletePartnersResponse {
    pub fn builder() -> AddressesDeletePartnersResponseBuilder {
        <AddressesDeletePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddressesDeletePartnersResponseBuilder {
    deleted: Option<bool>,
}

impl AddressesDeletePartnersResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddressesDeletePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](AddressesDeletePartnersResponseBuilder::deleted)
    pub fn build(self) -> Result<AddressesDeletePartnersResponse, BuildError> {
        Ok(AddressesDeletePartnersResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
