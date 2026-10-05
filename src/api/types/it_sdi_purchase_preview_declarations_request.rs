pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItSdiPurchasePreviewDeclarationsRequest {
    #[serde(rename = "purchaseInvoiceId")]
    #[serde(default)]
    pub purchase_invoice_id: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "tipoDocumento")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_documento: Option<ItSdiPurchasePreviewDeclarationsRequestTipoDocumento>,
}

impl ItSdiPurchasePreviewDeclarationsRequest {
    pub fn builder() -> ItSdiPurchasePreviewDeclarationsRequestBuilder {
        <ItSdiPurchasePreviewDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItSdiPurchasePreviewDeclarationsRequestBuilder {
    purchase_invoice_id: Option<String>,
    vat_rate_percent: Option<String>,
    tipo_documento: Option<ItSdiPurchasePreviewDeclarationsRequestTipoDocumento>,
}

impl ItSdiPurchasePreviewDeclarationsRequestBuilder {
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
        value: ItSdiPurchasePreviewDeclarationsRequestTipoDocumento,
    ) -> Self {
        self.tipo_documento = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItSdiPurchasePreviewDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`purchase_invoice_id`](ItSdiPurchasePreviewDeclarationsRequestBuilder::purchase_invoice_id)
    pub fn build(self) -> Result<ItSdiPurchasePreviewDeclarationsRequest, BuildError> {
        Ok(ItSdiPurchasePreviewDeclarationsRequest {
            purchase_invoice_id: self
                .purchase_invoice_id
                .ok_or_else(|| BuildError::missing_field("purchase_invoice_id"))?,
            vat_rate_percent: self.vat_rate_percent,
            tipo_documento: self.tipo_documento,
        })
    }
}
