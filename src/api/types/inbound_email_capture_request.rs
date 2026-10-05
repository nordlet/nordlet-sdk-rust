pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboundEmailCaptureRequest {
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_to: Option<String>,
    #[serde(rename = "ToFull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_full: Option<Vec<InboundEmailCaptureRequestToFullItem>>,
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_from: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_subject: Option<String>,
    #[serde(rename = "Attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_attachments: Option<Vec<InboundEmailCaptureRequestAttachmentsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<InboundEmailCaptureRequestTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<InboundEmailCaptureRequestAttachmentsItem>>,
}

impl InboundEmailCaptureRequest {
    pub fn builder() -> InboundEmailCaptureRequestBuilder {
        <InboundEmailCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboundEmailCaptureRequestBuilder {
    postmark_to: Option<String>,
    to_full: Option<Vec<InboundEmailCaptureRequestToFullItem>>,
    postmark_from: Option<String>,
    postmark_subject: Option<String>,
    postmark_attachments: Option<Vec<InboundEmailCaptureRequestAttachmentsItem>>,
    to: Option<InboundEmailCaptureRequestTo>,
    from: Option<String>,
    subject: Option<String>,
    attachments: Option<Vec<InboundEmailCaptureRequestAttachmentsItem>>,
}

impl InboundEmailCaptureRequestBuilder {
    pub fn postmark_to(mut self, value: impl Into<String>) -> Self {
        self.postmark_to = Some(value.into());
        self
    }

    pub fn to_full(mut self, value: Vec<InboundEmailCaptureRequestToFullItem>) -> Self {
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
        value: Vec<InboundEmailCaptureRequestAttachmentsItem>,
    ) -> Self {
        self.postmark_attachments = Some(value);
        self
    }

    pub fn to(mut self, value: InboundEmailCaptureRequestTo) -> Self {
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

    pub fn attachments(mut self, value: Vec<InboundEmailCaptureRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboundEmailCaptureRequest`].
    pub fn build(self) -> Result<InboundEmailCaptureRequest, BuildError> {
        Ok(InboundEmailCaptureRequest {
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
