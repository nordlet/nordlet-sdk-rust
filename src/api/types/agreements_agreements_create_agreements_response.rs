pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgreementsCreateAgreementsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "typeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    pub kind: AgreementsCreateAgreementsResponseKind,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(rename = "bankAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<String>,
    #[serde(default)]
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "autoRenew")]
    #[serde(default)]
    pub auto_renew: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "billingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<AgreementsCreateAgreementsResponseBillingPeriod>,
    #[serde(default)]
    pub currency: String,
    pub status: AgreementsCreateAgreementsResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub items: Vec<AgreementsCreateAgreementsResponseItemsItem>,
}

impl AgreementsCreateAgreementsResponse {
    pub fn builder() -> AgreementsCreateAgreementsResponseBuilder {
        <AgreementsCreateAgreementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsCreateAgreementsResponseBuilder {
    id: Option<String>,
    type_id: Option<String>,
    kind: Option<AgreementsCreateAgreementsResponseKind>,
    partner_id: Option<String>,
    employee_id: Option<String>,
    bank_account_id: Option<String>,
    number: Option<String>,
    name: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    auto_renew: Option<bool>,
    value: Option<String>,
    billing_period: Option<AgreementsCreateAgreementsResponseBillingPeriod>,
    currency: Option<String>,
    status: Option<AgreementsCreateAgreementsResponseStatus>,
    notes: Option<String>,
    document_ref: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    items: Option<Vec<AgreementsCreateAgreementsResponseItemsItem>>,
}

impl AgreementsCreateAgreementsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn type_id(mut self, value: impl Into<String>) -> Self {
        self.type_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: AgreementsCreateAgreementsResponseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn billing_period(
        mut self,
        value: AgreementsCreateAgreementsResponseBillingPeriod,
    ) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: AgreementsCreateAgreementsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<AgreementsCreateAgreementsResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgreementsCreateAgreementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgreementsCreateAgreementsResponseBuilder::id)
    /// - [`kind`](AgreementsCreateAgreementsResponseBuilder::kind)
    /// - [`number`](AgreementsCreateAgreementsResponseBuilder::number)
    /// - [`start_date`](AgreementsCreateAgreementsResponseBuilder::start_date)
    /// - [`auto_renew`](AgreementsCreateAgreementsResponseBuilder::auto_renew)
    /// - [`currency`](AgreementsCreateAgreementsResponseBuilder::currency)
    /// - [`status`](AgreementsCreateAgreementsResponseBuilder::status)
    /// - [`created_at`](AgreementsCreateAgreementsResponseBuilder::created_at)
    /// - [`items`](AgreementsCreateAgreementsResponseBuilder::items)
    pub fn build(self) -> Result<AgreementsCreateAgreementsResponse, BuildError> {
        Ok(AgreementsCreateAgreementsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            type_id: self.type_id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            partner_id: self.partner_id,
            employee_id: self.employee_id,
            bank_account_id: self.bank_account_id,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            name: self.name,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            auto_renew: self
                .auto_renew
                .ok_or_else(|| BuildError::missing_field("auto_renew"))?,
            value: self.value,
            billing_period: self.billing_period,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            document_ref: self.document_ref,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
