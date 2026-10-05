pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContactsDeletePartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl ContactsDeletePartnersRequest {
    pub fn builder() -> ContactsDeletePartnersRequestBuilder {
        <ContactsDeletePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactsDeletePartnersRequestBuilder {
    id: Option<String>,
}

impl ContactsDeletePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContactsDeletePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ContactsDeletePartnersRequestBuilder::id)
    pub fn build(self) -> Result<ContactsDeletePartnersRequest, BuildError> {
        Ok(ContactsDeletePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
