pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersInquiriesCreateRequest {
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(rename = "contactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(default)]
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "assignedUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1PartnersInquiriesCreateRequest {
    pub fn builder() -> PostV1PartnersInquiriesCreateRequestBuilder {
        <PostV1PartnersInquiriesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersInquiriesCreateRequestBuilder {
    partner_id: Option<String>,
    contact_name: Option<String>,
    contact_email: Option<String>,
    contact_phone: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    channel: Option<String>,
    assigned_user_id: Option<String>,
    notes: Option<String>,
}

impl PostV1PartnersInquiriesCreateRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(value.into());
        self
    }

    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(value.into());
        self
    }

    pub fn contact_phone(mut self, value: impl Into<String>) -> Self {
        self.contact_phone = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn assigned_user_id(mut self, value: impl Into<String>) -> Self {
        self.assigned_user_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersInquiriesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`subject`](PostV1PartnersInquiriesCreateRequestBuilder::subject)
    pub fn build(self) -> Result<PostV1PartnersInquiriesCreateRequest, BuildError> {
        Ok(PostV1PartnersInquiriesCreateRequest {
            partner_id: self.partner_id,
            contact_name: self.contact_name,
            contact_email: self.contact_email,
            contact_phone: self.contact_phone,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            body: self.body,
            channel: self.channel,
            assigned_user_id: self.assigned_user_id,
            notes: self.notes,
        })
    }
}
