pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsItSdiPurchaseSendRequest {
    #[serde(rename = "purchaseInvoiceId")]
    #[serde(default)]
    pub purchase_invoice_id: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "tipoDocumento")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_documento: Option<PostV1DeclarationsItSdiPurchaseSendRequestTipoDocumento>,
}

impl PostV1DeclarationsItSdiPurchaseSendRequest {
    pub fn builder() -> PostV1DeclarationsItSdiPurchaseSendRequestBuilder {
        <PostV1DeclarationsItSdiPurchaseSendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsItSdiPurchaseSendRequestBuilder {
    purchase_invoice_id: Option<String>,
    vat_rate_percent: Option<String>,
    tipo_documento: Option<PostV1DeclarationsItSdiPurchaseSendRequestTipoDocumento>,
}

impl PostV1DeclarationsItSdiPurchaseSendRequestBuilder {
    pub fn purchase_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_invoice_id = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn tipo_documento(
        mut self,
        value: PostV1DeclarationsItSdiPurchaseSendRequestTipoDocumento,
    ) -> Self {
        self.tipo_documento = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsItSdiPurchaseSendRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`purchase_invoice_id`](PostV1DeclarationsItSdiPurchaseSendRequestBuilder::purchase_invoice_id)
    pub fn build(self) -> Result<PostV1DeclarationsItSdiPurchaseSendRequest, BuildError> {
        Ok(PostV1DeclarationsItSdiPurchaseSendRequest {
            purchase_invoice_id: self
                .purchase_invoice_id
                .ok_or_else(|| BuildError::missing_field("purchase_invoice_id"))?,
            vat_rate_percent: self.vat_rate_percent,
            tipo_documento: self.tipo_documento,
        })
    }
}
