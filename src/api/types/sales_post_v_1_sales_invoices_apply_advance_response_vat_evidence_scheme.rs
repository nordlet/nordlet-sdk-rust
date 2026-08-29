pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme {
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

impl PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme {
    pub fn builder() -> PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceSchemeBuilder {
        <PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceSchemeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceSchemeBuilder {
    vat_scheme: Option<String>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
}

impl PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceSchemeBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deemed_supplier`](PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceSchemeBuilder::deemed_supplier)
    pub fn build(
        self,
    ) -> Result<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme, BuildError> {
        Ok(PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme {
            vat_scheme: self.vat_scheme,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self
                .deemed_supplier
                .ok_or_else(|| BuildError::missing_field("deemed_supplier"))?,
        })
    }
}
