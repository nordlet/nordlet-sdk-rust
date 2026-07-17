pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersContactsCreateRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
}

impl PostV1PartnersContactsCreateRequest {
    pub fn builder() -> PostV1PartnersContactsCreateRequestBuilder {
        <PostV1PartnersContactsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsCreateRequestBuilder {
    name: Option<String>,
    role: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    notes: Option<String>,
    partner_id: Option<String>,
}

impl PostV1PartnersContactsCreateRequestBuilder {
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

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PartnersContactsCreateRequestBuilder::name)
    /// - [`partner_id`](PostV1PartnersContactsCreateRequestBuilder::partner_id)
    pub fn build(self) -> Result<PostV1PartnersContactsCreateRequest, BuildError> {
        Ok(PostV1PartnersContactsCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role,
            email: self.email,
            phone: self.phone,
            notes: self.notes,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
        })
    }
}
