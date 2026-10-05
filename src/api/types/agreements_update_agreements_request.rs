pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsUpdateAgreementsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "typeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AgreementsUpdateAgreementsRequestKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "autoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "billingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<AgreementsUpdateAgreementsRequestBillingPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgreementsUpdateAgreementsRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
}

impl AgreementsUpdateAgreementsRequest {
    pub fn builder() -> AgreementsUpdateAgreementsRequestBuilder {
        <AgreementsUpdateAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsUpdateAgreementsRequestBuilder {
    id: Option<String>,
    type_id: Option<String>,
    kind: Option<AgreementsUpdateAgreementsRequestKind>,
    name: Option<String>,
    end_date: Option<NaiveDate>,
    auto_renew: Option<bool>,
    value: Option<String>,
    billing_period: Option<AgreementsUpdateAgreementsRequestBillingPeriod>,
    status: Option<AgreementsUpdateAgreementsRequestStatus>,
    notes: Option<String>,
    document_ref: Option<String>,
}

impl AgreementsUpdateAgreementsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn type_id(mut self, value: impl Into<String>) -> Self {
        self.type_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: AgreementsUpdateAgreementsRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn billing_period(mut self, value: AgreementsUpdateAgreementsRequestBillingPeriod) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn status(mut self, value: AgreementsUpdateAgreementsRequestStatus) -> Self {
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

    /// Consumes the builder and constructs a [`AgreementsUpdateAgreementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgreementsUpdateAgreementsRequestBuilder::id)
    pub fn build(self) -> Result<AgreementsUpdateAgreementsRequest, BuildError> {
        Ok(AgreementsUpdateAgreementsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            type_id: self.type_id,
            kind: self.kind,
            name: self.name,
            end_date: self.end_date,
            auto_renew: self.auto_renew,
            value: self.value,
            billing_period: self.billing_period,
            status: self.status,
            notes: self.notes,
            document_ref: self.document_ref,
        })
    }
}
