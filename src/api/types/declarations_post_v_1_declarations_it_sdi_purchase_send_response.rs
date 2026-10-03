pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsItSdiPurchaseSendResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(default)]
    pub system: String,
    pub transport: PostV1DeclarationsItSdiPurchaseSendResponseTransport,
    #[serde(rename = "tipoDocumento")]
    pub tipo_documento: PostV1DeclarationsItSdiPurchaseSendResponseTipoDocumento,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "nationalNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_number: Option<String>,
    pub status: PostV1DeclarationsItSdiPurchaseSendResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsItSdiPurchaseSendResponse {
    pub fn builder() -> PostV1DeclarationsItSdiPurchaseSendResponseBuilder {
        <PostV1DeclarationsItSdiPurchaseSendResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsItSdiPurchaseSendResponseBuilder {
    sent: Option<bool>,
    system: Option<String>,
    transport: Option<PostV1DeclarationsItSdiPurchaseSendResponseTransport>,
    tipo_documento: Option<PostV1DeclarationsItSdiPurchaseSendResponseTipoDocumento>,
    message_id: Option<String>,
    national_number: Option<String>,
    status: Option<PostV1DeclarationsItSdiPurchaseSendResponseStatus>,
    detail: Option<String>,
    file_id: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsItSdiPurchaseSendResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn transport(
        mut self,
        value: PostV1DeclarationsItSdiPurchaseSendResponseTransport,
    ) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn tipo_documento(
        mut self,
        value: PostV1DeclarationsItSdiPurchaseSendResponseTipoDocumento,
    ) -> Self {
        self.tipo_documento = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn national_number(mut self, value: impl Into<String>) -> Self {
        self.national_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1DeclarationsItSdiPurchaseSendResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsItSdiPurchaseSendResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::sent)
    /// - [`system`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::system)
    /// - [`transport`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::transport)
    /// - [`tipo_documento`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::tipo_documento)
    /// - [`message_id`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::message_id)
    /// - [`status`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::status)
    /// - [`file_id`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::file_id)
    /// - [`net`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::net)
    /// - [`vat`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::vat)
    /// - [`warnings`](PostV1DeclarationsItSdiPurchaseSendResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsItSdiPurchaseSendResponse, BuildError> {
        Ok(PostV1DeclarationsItSdiPurchaseSendResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            transport: self
                .transport
                .ok_or_else(|| BuildError::missing_field("transport"))?,
            tipo_documento: self
                .tipo_documento
                .ok_or_else(|| BuildError::missing_field("tipo_documento"))?,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            national_number: self.national_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            detail: self.detail,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
