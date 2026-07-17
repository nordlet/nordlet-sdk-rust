pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersContactsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
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
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1PartnersContactsCreateResponse {
    pub fn builder() -> PostV1PartnersContactsCreateResponseBuilder {
        <PostV1PartnersContactsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsCreateResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    name: Option<String>,
    role: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1PartnersContactsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersContactsCreateResponseBuilder::id)
    /// - [`partner_id`](PostV1PartnersContactsCreateResponseBuilder::partner_id)
    /// - [`name`](PostV1PartnersContactsCreateResponseBuilder::name)
    /// - [`created_at`](PostV1PartnersContactsCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1PartnersContactsCreateResponse, BuildError> {
        Ok(PostV1PartnersContactsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role,
            email: self.email,
            phone: self.phone,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
