pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditCheckPartnersRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "additionalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_amount: Option<String>,
}

impl CreditCheckPartnersRequest {
    pub fn builder() -> CreditCheckPartnersRequestBuilder {
        <CreditCheckPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditCheckPartnersRequestBuilder {
    partner_id: Option<String>,
    additional_amount: Option<String>,
}

impl CreditCheckPartnersRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn additional_amount(mut self, value: impl Into<String>) -> Self {
        self.additional_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditCheckPartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](CreditCheckPartnersRequestBuilder::partner_id)
    pub fn build(self) -> Result<CreditCheckPartnersRequest, BuildError> {
        Ok(CreditCheckPartnersRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            additional_amount: self.additional_amount,
        })
    }
}
