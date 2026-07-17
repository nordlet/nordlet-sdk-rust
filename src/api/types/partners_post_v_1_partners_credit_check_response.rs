pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersCreditCheckResponse {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "creditLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_limit: Option<String>,
    #[serde(rename = "openReceivables")]
    #[serde(default)]
    pub open_receivables: String,
    #[serde(rename = "additionalAmount")]
    #[serde(default)]
    pub additional_amount: String,
    #[serde(rename = "totalExposure")]
    #[serde(default)]
    pub total_exposure: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<String>,
    #[serde(default)]
    pub exceeded: bool,
}

impl PostV1PartnersCreditCheckResponse {
    pub fn builder() -> PostV1PartnersCreditCheckResponseBuilder {
        <PostV1PartnersCreditCheckResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersCreditCheckResponseBuilder {
    partner_id: Option<String>,
    partner_name: Option<String>,
    credit_limit: Option<String>,
    open_receivables: Option<String>,
    additional_amount: Option<String>,
    total_exposure: Option<String>,
    available: Option<String>,
    exceeded: Option<bool>,
}

impl PostV1PartnersCreditCheckResponseBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn credit_limit(mut self, value: impl Into<String>) -> Self {
        self.credit_limit = Some(value.into());
        self
    }

    pub fn open_receivables(mut self, value: impl Into<String>) -> Self {
        self.open_receivables = Some(value.into());
        self
    }

    pub fn additional_amount(mut self, value: impl Into<String>) -> Self {
        self.additional_amount = Some(value.into());
        self
    }

    pub fn total_exposure(mut self, value: impl Into<String>) -> Self {
        self.total_exposure = Some(value.into());
        self
    }

    pub fn available(mut self, value: impl Into<String>) -> Self {
        self.available = Some(value.into());
        self
    }

    pub fn exceeded(mut self, value: bool) -> Self {
        self.exceeded = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersCreditCheckResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1PartnersCreditCheckResponseBuilder::partner_id)
    /// - [`partner_name`](PostV1PartnersCreditCheckResponseBuilder::partner_name)
    /// - [`open_receivables`](PostV1PartnersCreditCheckResponseBuilder::open_receivables)
    /// - [`additional_amount`](PostV1PartnersCreditCheckResponseBuilder::additional_amount)
    /// - [`total_exposure`](PostV1PartnersCreditCheckResponseBuilder::total_exposure)
    /// - [`exceeded`](PostV1PartnersCreditCheckResponseBuilder::exceeded)
    pub fn build(self) -> Result<PostV1PartnersCreditCheckResponse, BuildError> {
        Ok(PostV1PartnersCreditCheckResponse {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            credit_limit: self.credit_limit,
            open_receivables: self
                .open_receivables
                .ok_or_else(|| BuildError::missing_field("open_receivables"))?,
            additional_amount: self
                .additional_amount
                .ok_or_else(|| BuildError::missing_field("additional_amount"))?,
            total_exposure: self
                .total_exposure
                .ok_or_else(|| BuildError::missing_field("total_exposure"))?,
            available: self.available,
            exceeded: self
                .exceeded
                .ok_or_else(|| BuildError::missing_field("exceeded"))?,
        })
    }
}
