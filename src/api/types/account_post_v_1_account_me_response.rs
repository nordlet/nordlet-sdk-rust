pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountMeResponse {
    #[serde(default)]
    pub user: PostV1AccountMeResponseUser,
    #[serde(default)]
    pub locale: String,
    #[serde(rename = "activeCompanyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_company_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub billing: PostV1AccountMeResponseBilling,
    #[serde(rename = "referralPoints")]
    #[serde(default)]
    pub referral_points: i64,
    #[serde(default)]
    pub consent: PostV1AccountMeResponseConsent,
    #[serde(default)]
    pub companies: Vec<PostV1AccountMeResponseCompaniesItem>,
}

impl PostV1AccountMeResponse {
    pub fn builder() -> PostV1AccountMeResponseBuilder {
        <PostV1AccountMeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMeResponseBuilder {
    user: Option<PostV1AccountMeResponseUser>,
    locale: Option<String>,
    active_company_id: Option<String>,
    role: Option<String>,
    billing: Option<PostV1AccountMeResponseBilling>,
    referral_points: Option<i64>,
    consent: Option<PostV1AccountMeResponseConsent>,
    companies: Option<Vec<PostV1AccountMeResponseCompaniesItem>>,
}

impl PostV1AccountMeResponseBuilder {
    pub fn user(mut self, value: PostV1AccountMeResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn active_company_id(mut self, value: impl Into<String>) -> Self {
        self.active_company_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn billing(mut self, value: PostV1AccountMeResponseBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn referral_points(mut self, value: i64) -> Self {
        self.referral_points = Some(value);
        self
    }

    pub fn consent(mut self, value: PostV1AccountMeResponseConsent) -> Self {
        self.consent = Some(value);
        self
    }

    pub fn companies(mut self, value: Vec<PostV1AccountMeResponseCompaniesItem>) -> Self {
        self.companies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user`](PostV1AccountMeResponseBuilder::user)
    /// - [`locale`](PostV1AccountMeResponseBuilder::locale)
    /// - [`billing`](PostV1AccountMeResponseBuilder::billing)
    /// - [`referral_points`](PostV1AccountMeResponseBuilder::referral_points)
    /// - [`consent`](PostV1AccountMeResponseBuilder::consent)
    /// - [`companies`](PostV1AccountMeResponseBuilder::companies)
    pub fn build(self) -> Result<PostV1AccountMeResponse, BuildError> {
        Ok(PostV1AccountMeResponse {
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
            active_company_id: self.active_company_id,
            role: self.role,
            billing: self
                .billing
                .ok_or_else(|| BuildError::missing_field("billing"))?,
            referral_points: self
                .referral_points
                .ok_or_else(|| BuildError::missing_field("referral_points"))?,
            consent: self
                .consent
                .ok_or_else(|| BuildError::missing_field("consent"))?,
            companies: self
                .companies
                .ok_or_else(|| BuildError::missing_field("companies"))?,
        })
    }
}
