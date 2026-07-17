pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersCreditCheckRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "additionalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_amount: Option<String>,
}

impl PostV1PartnersCreditCheckRequest {
    pub fn builder() -> PostV1PartnersCreditCheckRequestBuilder {
        <PostV1PartnersCreditCheckRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersCreditCheckRequestBuilder {
    partner_id: Option<String>,
    additional_amount: Option<String>,
}

impl PostV1PartnersCreditCheckRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn additional_amount(mut self, value: impl Into<String>) -> Self {
        self.additional_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersCreditCheckRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1PartnersCreditCheckRequestBuilder::partner_id)
    pub fn build(self) -> Result<PostV1PartnersCreditCheckRequest, BuildError> {
        Ok(PostV1PartnersCreditCheckRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            additional_amount: self.additional_amount,
        })
    }
}
