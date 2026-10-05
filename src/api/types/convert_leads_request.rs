pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertLeadsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_type: Option<ConvertLeadsRequestPartnerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
}

impl ConvertLeadsRequest {
    pub fn builder() -> ConvertLeadsRequestBuilder {
        <ConvertLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertLeadsRequestBuilder {
    id: Option<String>,
    partner_type: Option<ConvertLeadsRequestPartnerType>,
    code: Option<String>,
    vat_code: Option<String>,
}

impl ConvertLeadsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_type(mut self, value: ConvertLeadsRequestPartnerType) -> Self {
        self.partner_type = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConvertLeadsRequestBuilder::id)
    pub fn build(self) -> Result<ConvertLeadsRequest, BuildError> {
        Ok(ConvertLeadsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_type: self.partner_type,
            code: self.code,
            vat_code: self.vat_code,
        })
    }
}
