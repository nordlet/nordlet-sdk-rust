pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PartnersInquiriesListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
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
    #[serde(default)]
    pub channel: String,
    pub status: PostV1PartnersInquiriesListResponseRowsItemStatus,
    #[serde(rename = "assignedUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(rename = "closedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
}

impl PostV1PartnersInquiriesListResponseRowsItem {
    pub fn builder() -> PostV1PartnersInquiriesListResponseRowsItemBuilder {
        <PostV1PartnersInquiriesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersInquiriesListResponseRowsItemBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    contact_name: Option<String>,
    contact_email: Option<String>,
    contact_phone: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    channel: Option<String>,
    status: Option<PostV1PartnersInquiriesListResponseRowsItemStatus>,
    assigned_user_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    closed_at: Option<String>,
}

impl PostV1PartnersInquiriesListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
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

    pub fn status(mut self, value: PostV1PartnersInquiriesListResponseRowsItemStatus) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn closed_at(mut self, value: impl Into<String>) -> Self {
        self.closed_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersInquiriesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersInquiriesListResponseRowsItemBuilder::id)
    /// - [`subject`](PostV1PartnersInquiriesListResponseRowsItemBuilder::subject)
    /// - [`channel`](PostV1PartnersInquiriesListResponseRowsItemBuilder::channel)
    /// - [`status`](PostV1PartnersInquiriesListResponseRowsItemBuilder::status)
    /// - [`created_at`](PostV1PartnersInquiriesListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1PartnersInquiriesListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1PartnersInquiriesListResponseRowsItem, BuildError> {
        Ok(PostV1PartnersInquiriesListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            partner_name: self.partner_name,
            contact_name: self.contact_name,
            contact_email: self.contact_email,
            contact_phone: self.contact_phone,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            body: self.body,
            channel: self
                .channel
                .ok_or_else(|| BuildError::missing_field("channel"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            assigned_user_id: self.assigned_user_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            closed_at: self.closed_at,
        })
    }
}
