pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItSdiPurchaseSendDeclarationsResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(default)]
    pub system: String,
    pub transport: ItSdiPurchaseSendDeclarationsResponseTransport,
    #[serde(rename = "tipoDocumento")]
    pub tipo_documento: ItSdiPurchaseSendDeclarationsResponseTipoDocumento,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "nationalNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_number: Option<String>,
    pub status: ItSdiPurchaseSendDeclarationsResponseStatus,
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

impl ItSdiPurchaseSendDeclarationsResponse {
    pub fn builder() -> ItSdiPurchaseSendDeclarationsResponseBuilder {
        <ItSdiPurchaseSendDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItSdiPurchaseSendDeclarationsResponseBuilder {
    sent: Option<bool>,
    system: Option<String>,
    transport: Option<ItSdiPurchaseSendDeclarationsResponseTransport>,
    tipo_documento: Option<ItSdiPurchaseSendDeclarationsResponseTipoDocumento>,
    message_id: Option<String>,
    national_number: Option<String>,
    status: Option<ItSdiPurchaseSendDeclarationsResponseStatus>,
    detail: Option<String>,
    file_id: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    warnings: Option<Vec<String>>,
}

impl ItSdiPurchaseSendDeclarationsResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn transport(mut self, value: ItSdiPurchaseSendDeclarationsResponseTransport) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn tipo_documento(
        mut self,
        value: ItSdiPurchaseSendDeclarationsResponseTipoDocumento,
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

    pub fn status(mut self, value: ItSdiPurchaseSendDeclarationsResponseStatus) -> Self {
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

    /// Consumes the builder and constructs a [`ItSdiPurchaseSendDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](ItSdiPurchaseSendDeclarationsResponseBuilder::sent)
    /// - [`system`](ItSdiPurchaseSendDeclarationsResponseBuilder::system)
    /// - [`transport`](ItSdiPurchaseSendDeclarationsResponseBuilder::transport)
    /// - [`tipo_documento`](ItSdiPurchaseSendDeclarationsResponseBuilder::tipo_documento)
    /// - [`message_id`](ItSdiPurchaseSendDeclarationsResponseBuilder::message_id)
    /// - [`status`](ItSdiPurchaseSendDeclarationsResponseBuilder::status)
    /// - [`file_id`](ItSdiPurchaseSendDeclarationsResponseBuilder::file_id)
    /// - [`net`](ItSdiPurchaseSendDeclarationsResponseBuilder::net)
    /// - [`vat`](ItSdiPurchaseSendDeclarationsResponseBuilder::vat)
    /// - [`warnings`](ItSdiPurchaseSendDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<ItSdiPurchaseSendDeclarationsResponse, BuildError> {
        Ok(ItSdiPurchaseSendDeclarationsResponse {
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
