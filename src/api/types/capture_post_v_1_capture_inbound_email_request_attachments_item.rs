pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureInboundEmailRequestAttachmentsItem {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_name: Option<String>,
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_content: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmark_content_type: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl PostV1CaptureInboundEmailRequestAttachmentsItem {
    pub fn builder() -> PostV1CaptureInboundEmailRequestAttachmentsItemBuilder {
        <PostV1CaptureInboundEmailRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureInboundEmailRequestAttachmentsItemBuilder {
    postmark_name: Option<String>,
    postmark_content: Option<String>,
    postmark_content_type: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    content: Option<String>,
}

impl PostV1CaptureInboundEmailRequestAttachmentsItemBuilder {
    pub fn postmark_name(mut self, value: impl Into<String>) -> Self {
        self.postmark_name = Some(value.into());
        self
    }

    pub fn postmark_content(mut self, value: impl Into<String>) -> Self {
        self.postmark_content = Some(value.into());
        self
    }

    pub fn postmark_content_type(mut self, value: impl Into<String>) -> Self {
        self.postmark_content_type = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureInboundEmailRequestAttachmentsItem`].
    pub fn build(self) -> Result<PostV1CaptureInboundEmailRequestAttachmentsItem, BuildError> {
        Ok(PostV1CaptureInboundEmailRequestAttachmentsItem {
            postmark_name: self.postmark_name,
            postmark_content: self.postmark_content,
            postmark_content_type: self.postmark_content_type,
            file_name: self.file_name,
            mime_type: self.mime_type,
            content: self.content,
        })
    }
}
