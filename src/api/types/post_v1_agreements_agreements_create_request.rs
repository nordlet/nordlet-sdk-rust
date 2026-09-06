pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsCreateRequest {
    #[serde(rename = "typeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<PostV1AgreementsAgreementsCreateRequestKind>,
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
    pub start_date: String,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "autoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "billingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<PostV1AgreementsAgreementsCreateRequestBillingPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostV1AgreementsAgreementsCreateRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PostV1AgreementsAgreementsCreateRequestItemsItem>>,
}

impl PostV1AgreementsAgreementsCreateRequest {
    pub fn builder() -> PostV1AgreementsAgreementsCreateRequestBuilder {
        <PostV1AgreementsAgreementsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsCreateRequestBuilder {
    type_id: Option<String>,
    kind: Option<PostV1AgreementsAgreementsCreateRequestKind>,
    partner_id: Option<String>,
    employee_id: Option<String>,
    bank_account_id: Option<String>,
    number: Option<String>,
    name: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    auto_renew: Option<bool>,
    value: Option<String>,
    billing_period: Option<PostV1AgreementsAgreementsCreateRequestBillingPeriod>,
    currency: Option<String>,
    status: Option<PostV1AgreementsAgreementsCreateRequestStatus>,
    notes: Option<String>,
    document_ref: Option<String>,
    items: Option<Vec<PostV1AgreementsAgreementsCreateRequestItemsItem>>,
}

impl PostV1AgreementsAgreementsCreateRequestBuilder {
    pub fn type_id(mut self, value: impl Into<String>) -> Self {
        self.type_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1AgreementsAgreementsCreateRequestKind) -> Self {
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

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
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
        value: PostV1AgreementsAgreementsCreateRequestBillingPeriod,
    ) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1AgreementsAgreementsCreateRequestStatus) -> Self {
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

    pub fn items(mut self, value: Vec<PostV1AgreementsAgreementsCreateRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`number`](PostV1AgreementsAgreementsCreateRequestBuilder::number)
    /// - [`start_date`](PostV1AgreementsAgreementsCreateRequestBuilder::start_date)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsCreateRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsCreateRequest {
            type_id: self.type_id,
            kind: self.kind,
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
            auto_renew: self.auto_renew,
            value: self.value,
            billing_period: self.billing_period,
            currency: self.currency,
            status: self.status,
            notes: self.notes,
            document_ref: self.document_ref,
            items: self.items,
        })
    }
}
