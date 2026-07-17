pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersContactsUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersContactsUpdateRequest {
    pub fn builder() -> PostV1PartnersContactsUpdateRequestBuilder {
        <PostV1PartnersContactsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsUpdateRequestBuilder {
    name: Option<String>,
    role: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    notes: Option<String>,
    id: Option<String>,
}

impl PostV1PartnersContactsUpdateRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersContactsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersContactsUpdateRequest, BuildError> {
        Ok(PostV1PartnersContactsUpdateRequest {
            name: self.name,
            role: self.role,
            email: self.email,
            phone: self.phone,
            notes: self.notes,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
