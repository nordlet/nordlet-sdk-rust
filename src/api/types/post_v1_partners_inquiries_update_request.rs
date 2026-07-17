pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersInquiriesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostV1PartnersInquiriesUpdateRequestStatus>,
    #[serde(rename = "assignedUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1PartnersInquiriesUpdateRequest {
    pub fn builder() -> PostV1PartnersInquiriesUpdateRequestBuilder {
        <PostV1PartnersInquiriesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersInquiriesUpdateRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    channel: Option<String>,
    status: Option<PostV1PartnersInquiriesUpdateRequestStatus>,
    assigned_user_id: Option<String>,
    notes: Option<String>,
}

impl PostV1PartnersInquiriesUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
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

    pub fn status(mut self, value: PostV1PartnersInquiriesUpdateRequestStatus) -> Self {
        self.status = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1PartnersInquiriesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersInquiriesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersInquiriesUpdateRequest, BuildError> {
        Ok(PostV1PartnersInquiriesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            subject: self.subject,
            body: self.body,
            channel: self.channel,
            status: self.status,
            assigned_user_id: self.assigned_user_id,
            notes: self.notes,
        })
    }
}
