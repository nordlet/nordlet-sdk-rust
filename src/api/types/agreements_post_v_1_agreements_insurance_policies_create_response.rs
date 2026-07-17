pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsInsurancePoliciesCreateResponse {
    #[serde(default)]
    pub id: String,
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
    #[serde(default)]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AgreementsInsurancePoliciesCreateResponse {
    pub fn builder() -> PostV1AgreementsInsurancePoliciesCreateResponseBuilder {
        <PostV1AgreementsInsurancePoliciesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsInsurancePoliciesCreateResponseBuilder {
    id: Option<String>,
    insurer_partner_id: Option<String>,
    policy_number: Option<String>,
    insured_object: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    premium: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1AgreementsInsurancePoliciesCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsInsurancePoliciesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsInsurancePoliciesCreateResponseBuilder::id)
    /// - [`policy_number`](PostV1AgreementsInsurancePoliciesCreateResponseBuilder::policy_number)
    /// - [`insured_object`](PostV1AgreementsInsurancePoliciesCreateResponseBuilder::insured_object)
    /// - [`from_date`](PostV1AgreementsInsurancePoliciesCreateResponseBuilder::from_date)
    /// - [`to_date`](PostV1AgreementsInsurancePoliciesCreateResponseBuilder::to_date)
    /// - [`currency`](PostV1AgreementsInsurancePoliciesCreateResponseBuilder::currency)
    /// - [`created_at`](PostV1AgreementsInsurancePoliciesCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1AgreementsInsurancePoliciesCreateResponse, BuildError> {
        Ok(PostV1AgreementsInsurancePoliciesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
