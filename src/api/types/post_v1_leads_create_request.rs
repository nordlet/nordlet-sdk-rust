pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsCreateRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostV1LeadsCreateRequestStatus>,
    #[serde(rename = "estimatedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "assignedUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<PostV1LeadsCreateRequestDocumentsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
}

impl PostV1LeadsCreateRequest {
    pub fn builder() -> PostV1LeadsCreateRequestBuilder {
        <PostV1LeadsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsCreateRequestBuilder {
    name: Option<String>,
    contact_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    country_code: Option<String>,
    source_id: Option<String>,
    status: Option<PostV1LeadsCreateRequestStatus>,
    estimated_value: Option<String>,
    currency: Option<String>,
    description: Option<String>,
    assigned_user_id: Option<String>,
    documents: Option<Vec<PostV1LeadsCreateRequestDocumentsItem>>,
    notes: Option<Vec<String>>,
}

impl PostV1LeadsCreateRequestBuilder {
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

    pub fn status(mut self, value: PostV1LeadsCreateRequestStatus) -> Self {
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

    pub fn documents(mut self, value: Vec<PostV1LeadsCreateRequestDocumentsItem>) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LeadsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1LeadsCreateRequest, BuildError> {
        Ok(PostV1LeadsCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            contact_name: self.contact_name,
            email: self.email,
            phone: self.phone,
            website: self.website,
            country_code: self.country_code,
            source_id: self.source_id,
            status: self.status,
            estimated_value: self.estimated_value,
            currency: self.currency,
            description: self.description,
            assigned_user_id: self.assigned_user_id,
            documents: self.documents,
            notes: self.notes,
        })
    }
}
