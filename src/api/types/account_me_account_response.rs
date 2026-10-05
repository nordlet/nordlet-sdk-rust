pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MeAccountResponse {
    #[serde(default)]
    pub user: MeAccountResponseUser,
    #[serde(default)]
    pub locale: String,
    #[serde(rename = "activeCompanyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_company_id: Option<String>,
    #[serde(rename = "timeZone")]
    #[serde(default)]
    pub time_zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub billing: MeAccountResponseBilling,
    #[serde(rename = "referralPoints")]
    #[serde(default)]
    pub referral_points: i64,
    #[serde(default)]
    pub consent: MeAccountResponseConsent,
    #[serde(default)]
    pub companies: Vec<MeAccountResponseCompaniesItem>,
}

impl MeAccountResponse {
    pub fn builder() -> MeAccountResponseBuilder {
        <MeAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeAccountResponseBuilder {
    user: Option<MeAccountResponseUser>,
    locale: Option<String>,
    active_company_id: Option<String>,
    time_zone: Option<String>,
    role: Option<String>,
    billing: Option<MeAccountResponseBilling>,
    referral_points: Option<i64>,
    consent: Option<MeAccountResponseConsent>,
    companies: Option<Vec<MeAccountResponseCompaniesItem>>,
}

impl MeAccountResponseBuilder {
    pub fn user(mut self, value: MeAccountResponseUser) -> Self {
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

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn billing(mut self, value: MeAccountResponseBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn referral_points(mut self, value: i64) -> Self {
        self.referral_points = Some(value);
        self
    }

    pub fn consent(mut self, value: MeAccountResponseConsent) -> Self {
        self.consent = Some(value);
        self
    }

    pub fn companies(mut self, value: Vec<MeAccountResponseCompaniesItem>) -> Self {
        self.companies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user`](MeAccountResponseBuilder::user)
    /// - [`locale`](MeAccountResponseBuilder::locale)
    /// - [`time_zone`](MeAccountResponseBuilder::time_zone)
    /// - [`billing`](MeAccountResponseBuilder::billing)
    /// - [`referral_points`](MeAccountResponseBuilder::referral_points)
    /// - [`consent`](MeAccountResponseBuilder::consent)
    /// - [`companies`](MeAccountResponseBuilder::companies)
    pub fn build(self) -> Result<MeAccountResponse, BuildError> {
        Ok(MeAccountResponse {
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
            active_company_id: self.active_company_id,
            time_zone: self
                .time_zone
                .ok_or_else(|| BuildError::missing_field("time_zone"))?,
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
