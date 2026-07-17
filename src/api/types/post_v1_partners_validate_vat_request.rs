pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersValidateVatRequest {
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
}

impl PostV1PartnersValidateVatRequest {
    pub fn builder() -> PostV1PartnersValidateVatRequestBuilder {
        <PostV1PartnersValidateVatRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersValidateVatRequestBuilder {
    vat_code: Option<String>,
    partner_id: Option<String>,
}

impl PostV1PartnersValidateVatRequestBuilder {
    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersValidateVatRequest`].
    pub fn build(self) -> Result<PostV1PartnersValidateVatRequest, BuildError> {
        Ok(PostV1PartnersValidateVatRequest {
            vat_code: self.vat_code,
            partner_id: self.partner_id,
        })
    }
}
