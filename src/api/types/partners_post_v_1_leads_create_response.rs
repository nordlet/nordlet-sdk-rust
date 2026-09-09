pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LeadsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    pub status: PostV1LeadsCreateResponseStatus,
    #[serde(rename = "estimatedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_value: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "assignedUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "convertedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub converted_at: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1LeadsCreateResponse {
    pub fn builder() -> PostV1LeadsCreateResponseBuilder {
        <PostV1LeadsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    contact_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    country_code: Option<String>,
    source_id: Option<String>,
    source_name: Option<String>,
    status: Option<PostV1LeadsCreateResponseStatus>,
    estimated_value: Option<String>,
    currency: Option<String>,
    description: Option<String>,
    assigned_user_id: Option<String>,
    partner_id: Option<String>,
    converted_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1LeadsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(value.into());
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

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn source_name(mut self, value: impl Into<String>) -> Self {
        self.source_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1LeadsCreateResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn estimated_value(mut self, value: impl Into<String>) -> Self {
        self.estimated_value = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn assigned_user_id(mut self, value: impl Into<String>) -> Self {
        self.assigned_user_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn converted_at(mut self, value: impl Into<String>) -> Self {
        self.converted_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsCreateResponseBuilder::id)
    /// - [`name`](PostV1LeadsCreateResponseBuilder::name)
    /// - [`status`](PostV1LeadsCreateResponseBuilder::status)
    /// - [`currency`](PostV1LeadsCreateResponseBuilder::currency)
    /// - [`created_at`](PostV1LeadsCreateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1LeadsCreateResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1LeadsCreateResponse, BuildError> {
        Ok(PostV1LeadsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            contact_name: self.contact_name,
            email: self.email,
            phone: self.phone,
            website: self.website,
            country_code: self.country_code,
            source_id: self.source_id,
            source_name: self.source_name,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            estimated_value: self.estimated_value,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            assigned_user_id: self.assigned_user_id,
            partner_id: self.partner_id,
            converted_at: self.converted_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
