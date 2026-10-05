pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidateVatPartnersRequest {
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
}

impl ValidateVatPartnersRequest {
    pub fn builder() -> ValidateVatPartnersRequestBuilder {
        <ValidateVatPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateVatPartnersRequestBuilder {
    vat_code: Option<String>,
    partner_id: Option<String>,
}

impl ValidateVatPartnersRequestBuilder {
    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidateVatPartnersRequest`].
    pub fn build(self) -> Result<ValidateVatPartnersRequest, BuildError> {
        Ok(ValidateVatPartnersRequest {
            vat_code: self.vat_code,
            partner_id: self.partner_id,
        })
    }
}
