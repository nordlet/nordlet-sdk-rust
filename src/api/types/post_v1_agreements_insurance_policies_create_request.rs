pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsInsurancePoliciesCreateRequest {
    #[serde(rename = "insurerPartnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurer_partner_id: Option<String>,
    #[serde(rename = "policyNumber")]
    #[serde(default)]
    pub policy_number: String,
    #[serde(rename = "insuredObject")]
    #[serde(default)]
    pub insured_object: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1AgreementsInsurancePoliciesCreateRequest {
    pub fn builder() -> PostV1AgreementsInsurancePoliciesCreateRequestBuilder {
        <PostV1AgreementsInsurancePoliciesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsInsurancePoliciesCreateRequestBuilder {
    insurer_partner_id: Option<String>,
    policy_number: Option<String>,
    insured_object: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    premium: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
}

impl PostV1AgreementsInsurancePoliciesCreateRequestBuilder {
    pub fn insurer_partner_id(mut self, value: impl Into<String>) -> Self {
        self.insurer_partner_id = Some(value.into());
        self
    }

    pub fn policy_number(mut self, value: impl Into<String>) -> Self {
        self.policy_number = Some(value.into());
        self
    }

    pub fn insured_object(mut self, value: impl Into<String>) -> Self {
        self.insured_object = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn premium(mut self, value: impl Into<String>) -> Self {
        self.premium = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsInsurancePoliciesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`policy_number`](PostV1AgreementsInsurancePoliciesCreateRequestBuilder::policy_number)
    /// - [`insured_object`](PostV1AgreementsInsurancePoliciesCreateRequestBuilder::insured_object)
    /// - [`from_date`](PostV1AgreementsInsurancePoliciesCreateRequestBuilder::from_date)
    /// - [`to_date`](PostV1AgreementsInsurancePoliciesCreateRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1AgreementsInsurancePoliciesCreateRequest, BuildError> {
        Ok(PostV1AgreementsInsurancePoliciesCreateRequest {
            insurer_partner_id: self.insurer_partner_id,
            policy_number: self
                .policy_number
                .ok_or_else(|| BuildError::missing_field("policy_number"))?,
            insured_object: self
                .insured_object
                .ok_or_else(|| BuildError::missing_field("insured_object"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            premium: self.premium,
            currency: self.currency,
            notes: self.notes,
        })
    }
}
