pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesGetSalesResponseVatEvidenceScheme {
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<String>,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "deemedSupplier")]
    #[serde(default)]
    pub deemed_supplier: bool,
}

impl InvoicesGetSalesResponseVatEvidenceScheme {
    pub fn builder() -> InvoicesGetSalesResponseVatEvidenceSchemeBuilder {
        <InvoicesGetSalesResponseVatEvidenceSchemeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetSalesResponseVatEvidenceSchemeBuilder {
    vat_scheme: Option<String>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
}

impl InvoicesGetSalesResponseVatEvidenceSchemeBuilder {
    pub fn vat_scheme(mut self, value: impl Into<String>) -> Self {
        self.vat_scheme = Some(value.into());
        self
    }

    pub fn vat_country_code(mut self, value: impl Into<String>) -> Self {
        self.vat_country_code = Some(value.into());
        self
    }

    pub fn deemed_supplier(mut self, value: bool) -> Self {
        self.deemed_supplier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesGetSalesResponseVatEvidenceScheme`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deemed_supplier`](InvoicesGetSalesResponseVatEvidenceSchemeBuilder::deemed_supplier)
    pub fn build(self) -> Result<InvoicesGetSalesResponseVatEvidenceScheme, BuildError> {
        Ok(InvoicesGetSalesResponseVatEvidenceScheme {
            vat_scheme: self.vat_scheme,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self
                .deemed_supplier
                .ok_or_else(|| BuildError::missing_field("deemed_supplier"))?,
        })
    }
}
