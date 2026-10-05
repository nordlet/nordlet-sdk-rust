pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContactsDeletePartnersResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl ContactsDeletePartnersResponse {
    pub fn builder() -> ContactsDeletePartnersResponseBuilder {
        <ContactsDeletePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactsDeletePartnersResponseBuilder {
    deleted: Option<bool>,
}

impl ContactsDeletePartnersResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContactsDeletePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](ContactsDeletePartnersResponseBuilder::deleted)
    pub fn build(self) -> Result<ContactsDeletePartnersResponse, BuildError> {
        Ok(ContactsDeletePartnersResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
