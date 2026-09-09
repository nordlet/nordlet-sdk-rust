pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1CaptureInboundEmailRequest {
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_to: Option<String>,
    #[serde(rename = "ToFull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_full: Option<Vec<PostV1CaptureInboundEmailRequestToFullItem>>,
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_from: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_subject: Option<String>,
    #[serde(rename = "Attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_attachments: Option<Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<PostV1CaptureInboundEmailRequestTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>>,
}

impl PostV1CaptureInboundEmailRequest {
    pub fn builder() -> PostV1CaptureInboundEmailRequestBuilder {
        <PostV1CaptureInboundEmailRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureInboundEmailRequestBuilder {
    postmark_to: Option<String>,
    to_full: Option<Vec<PostV1CaptureInboundEmailRequestToFullItem>>,
    postmark_from: Option<String>,
    postmark_subject: Option<String>,
    postmark_attachments: Option<Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>>,
    to: Option<PostV1CaptureInboundEmailRequestTo>,
    from: Option<String>,
    subject: Option<String>,
    attachments: Option<Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>>,
}

impl PostV1CaptureInboundEmailRequestBuilder {
    pub fn postmark_to(mut self, value: impl Into<String>) -> Self {
        self.postmark_to = Some(value.into());
        self
    }

    pub fn to_full(mut self, value: Vec<PostV1CaptureInboundEmailRequestToFullItem>) -> Self {
        self.to_full = Some(value);
        self
    }

    pub fn postmark_from(mut self, value: impl Into<String>) -> Self {
        self.postmark_from = Some(value.into());
        self
    }

    pub fn postmark_subject(mut self, value: impl Into<String>) -> Self {
        self.postmark_subject = Some(value.into());
        self
    }

    pub fn postmark_attachments(
        mut self,
        value: Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>,
    ) -> Self {
        self.postmark_attachments = Some(value);
        self
    }

    pub fn to(mut self, value: PostV1CaptureInboundEmailRequestTo) -> Self {
        self.to = Some(value);
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn attachments(
        mut self,
        value: Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureInboundEmailRequest`].
    pub fn build(self) -> Result<PostV1CaptureInboundEmailRequest, BuildError> {
        Ok(PostV1CaptureInboundEmailRequest {
            postmark_to: self.postmark_to,
            to_full: self.to_full,
            postmark_from: self.postmark_from,
            postmark_subject: self.postmark_subject,
            postmark_attachments: self.postmark_attachments,
            to: self.to,
            from: self.from,
            subject: self.subject,
            attachments: self.attachments,
        })
    }
}
