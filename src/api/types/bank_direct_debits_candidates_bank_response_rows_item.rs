pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DirectDebitsCandidatesBankResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<NaiveDate>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "paidAmount")]
    #[serde(default)]
    pub paid_amount: String,
    #[serde(default)]
    pub remaining: String,
    #[serde(rename = "mandateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_id: Option<String>,
    #[serde(rename = "mandateReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,
    #[serde(rename = "mandateSignatureDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_signature_date: Option<NaiveDate>,
}

impl DirectDebitsCandidatesBankResponseRowsItem {
    pub fn builder() -> DirectDebitsCandidatesBankResponseRowsItemBuilder {
        <DirectDebitsCandidatesBankResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DirectDebitsCandidatesBankResponseRowsItemBuilder {
    id: Option<String>,
    full_number: Option<String>,
    issue_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    currency: Option<String>,
    gross_total: Option<String>,
    paid_amount: Option<String>,
    remaining: Option<String>,
    mandate_id: Option<String>,
    mandate_reference: Option<String>,
    mandate_signature_date: Option<NaiveDate>,
}

impl DirectDebitsCandidatesBankResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn paid_amount(mut self, value: impl Into<String>) -> Self {
        self.paid_amount = Some(value.into());
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn mandate_id(mut self, value: impl Into<String>) -> Self {
        self.mandate_id = Some(value.into());
        self
    }

    pub fn mandate_reference(mut self, value: impl Into<String>) -> Self {
        self.mandate_reference = Some(value.into());
        self
    }

    pub fn mandate_signature_date(mut self, value: NaiveDate) -> Self {
        self.mandate_signature_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DirectDebitsCandidatesBankResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DirectDebitsCandidatesBankResponseRowsItemBuilder::id)
    /// - [`partner_id`](DirectDebitsCandidatesBankResponseRowsItemBuilder::partner_id)
    /// - [`currency`](DirectDebitsCandidatesBankResponseRowsItemBuilder::currency)
    /// - [`gross_total`](DirectDebitsCandidatesBankResponseRowsItemBuilder::gross_total)
    /// - [`paid_amount`](DirectDebitsCandidatesBankResponseRowsItemBuilder::paid_amount)
    /// - [`remaining`](DirectDebitsCandidatesBankResponseRowsItemBuilder::remaining)
    pub fn build(self) -> Result<DirectDebitsCandidatesBankResponseRowsItem, BuildError> {
        Ok(DirectDebitsCandidatesBankResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            full_number: self.full_number,
            issue_date: self.issue_date,
            due_date: self.due_date,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self.partner_name,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            paid_amount: self
                .paid_amount
                .ok_or_else(|| BuildError::missing_field("paid_amount"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            mandate_id: self.mandate_id,
            mandate_reference: self.mandate_reference,
            mandate_signature_date: self.mandate_signature_date,
        })
    }
}
