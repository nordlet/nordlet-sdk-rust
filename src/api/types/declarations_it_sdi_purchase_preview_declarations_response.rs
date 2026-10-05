pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItSdiPurchasePreviewDeclarationsResponse {
    #[serde(rename = "tipoDocumento")]
    pub tipo_documento: ItSdiPurchasePreviewDeclarationsResponseTipoDocumento,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl ItSdiPurchasePreviewDeclarationsResponse {
    pub fn builder() -> ItSdiPurchasePreviewDeclarationsResponseBuilder {
        <ItSdiPurchasePreviewDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItSdiPurchasePreviewDeclarationsResponseBuilder {
    tipo_documento: Option<ItSdiPurchasePreviewDeclarationsResponseTipoDocumento>,
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    warnings: Option<Vec<String>>,
}

impl ItSdiPurchasePreviewDeclarationsResponseBuilder {
    pub fn tipo_documento(
        mut self,
        value: ItSdiPurchasePreviewDeclarationsResponseTipoDocumento,
    ) -> Self {
        self.tipo_documento = Some(value);
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
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

    /// Consumes the builder and constructs a [`ItSdiPurchasePreviewDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tipo_documento`](ItSdiPurchasePreviewDeclarationsResponseBuilder::tipo_documento)
    /// - [`file_name`](ItSdiPurchasePreviewDeclarationsResponseBuilder::file_name)
    /// - [`content_type`](ItSdiPurchasePreviewDeclarationsResponseBuilder::content_type)
    /// - [`data`](ItSdiPurchasePreviewDeclarationsResponseBuilder::data)
    /// - [`net`](ItSdiPurchasePreviewDeclarationsResponseBuilder::net)
    /// - [`vat`](ItSdiPurchasePreviewDeclarationsResponseBuilder::vat)
    /// - [`warnings`](ItSdiPurchasePreviewDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<ItSdiPurchasePreviewDeclarationsResponse, BuildError> {
        Ok(ItSdiPurchasePreviewDeclarationsResponse {
            tipo_documento: self
                .tipo_documento
                .ok_or_else(|| BuildError::missing_field("tipo_documento"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
