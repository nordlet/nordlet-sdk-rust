pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceVatResolveResponse {
    pub scheme: PostV1ReferenceVatResolveResponseScheme,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "reverseCharge")]
    #[serde(default)]
    pub reverse_charge: bool,
    #[serde(rename = "deemedSupplier")]
    #[serde(default)]
    pub deemed_supplier: bool,
    #[serde(rename = "zeroRated")]
    #[serde(default)]
    pub zero_rated: bool,
    #[serde(default)]
    pub rates: Vec<PostV1ReferenceVatResolveResponseRatesItem>,
    #[serde(rename = "legalBasis")]
    #[serde(default)]
    pub legal_basis: String,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1ReferenceVatResolveResponse {
    pub fn builder() -> PostV1ReferenceVatResolveResponseBuilder {
        <PostV1ReferenceVatResolveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatResolveResponseBuilder {
    scheme: Option<PostV1ReferenceVatResolveResponseScheme>,
    vat_country_code: Option<String>,
    reverse_charge: Option<bool>,
    deemed_supplier: Option<bool>,
    zero_rated: Option<bool>,
    rates: Option<Vec<PostV1ReferenceVatResolveResponseRatesItem>>,
    legal_basis: Option<String>,
    notes: Option<Vec<String>>,
}

impl PostV1ReferenceVatResolveResponseBuilder {
    pub fn scheme(mut self, value: PostV1ReferenceVatResolveResponseScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn vat_country_code(mut self, value: impl Into<String>) -> Self {
        self.vat_country_code = Some(value.into());
        self
    }

    pub fn reverse_charge(mut self, value: bool) -> Self {
        self.reverse_charge = Some(value);
        self
    }

    pub fn deemed_supplier(mut self, value: bool) -> Self {
        self.deemed_supplier = Some(value);
        self
    }

    pub fn zero_rated(mut self, value: bool) -> Self {
        self.zero_rated = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<PostV1ReferenceVatResolveResponseRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    pub fn legal_basis(mut self, value: impl Into<String>) -> Self {
        self.legal_basis = Some(value.into());
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceVatResolveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheme`](PostV1ReferenceVatResolveResponseBuilder::scheme)
    /// - [`reverse_charge`](PostV1ReferenceVatResolveResponseBuilder::reverse_charge)
    /// - [`deemed_supplier`](PostV1ReferenceVatResolveResponseBuilder::deemed_supplier)
    /// - [`zero_rated`](PostV1ReferenceVatResolveResponseBuilder::zero_rated)
    /// - [`rates`](PostV1ReferenceVatResolveResponseBuilder::rates)
    /// - [`legal_basis`](PostV1ReferenceVatResolveResponseBuilder::legal_basis)
    /// - [`notes`](PostV1ReferenceVatResolveResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1ReferenceVatResolveResponse, BuildError> {
        Ok(PostV1ReferenceVatResolveResponse {
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            vat_country_code: self.vat_country_code,
            reverse_charge: self
                .reverse_charge
                .ok_or_else(|| BuildError::missing_field("reverse_charge"))?,
            deemed_supplier: self
                .deemed_supplier
                .ok_or_else(|| BuildError::missing_field("deemed_supplier"))?,
            zero_rated: self
                .zero_rated
                .ok_or_else(|| BuildError::missing_field("zero_rated"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
            legal_basis: self
                .legal_basis
                .ok_or_else(|| BuildError::missing_field("legal_basis"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
