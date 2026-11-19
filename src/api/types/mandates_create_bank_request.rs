pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MandatesCreateBankRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    /// Sort code (Bacs) or routing number (ACH), for a mandate without an IBAN
    #[serde(rename = "bankCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Account number, for a mandate without an IBAN
    #[serde(rename = "bankAccountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    /// Checking or savings account; a US ACH file needs it
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<MandatesCreateBankRequestAccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<MandatesCreateBankRequestScheme>,
    #[serde(rename = "sequenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_type: Option<MandatesCreateBankRequestSequenceType>,
    #[serde(rename = "signatureDate")]
    #[serde(default)]
    pub signature_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "debtorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debtor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl MandatesCreateBankRequest {
    pub fn builder() -> MandatesCreateBankRequestBuilder {
        <MandatesCreateBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesCreateBankRequestBuilder {
    partner_id: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    bank_code: Option<String>,
    bank_account_number: Option<String>,
    account_type: Option<MandatesCreateBankRequestAccountType>,
    scheme: Option<MandatesCreateBankRequestScheme>,
    sequence_type: Option<MandatesCreateBankRequestSequenceType>,
    signature_date: Option<NaiveDate>,
    reference: Option<String>,
    debtor_name: Option<String>,
    notes: Option<String>,
}

impl MandatesCreateBankRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn bank_account_number(mut self, value: impl Into<String>) -> Self {
        self.bank_account_number = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: MandatesCreateBankRequestAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn scheme(mut self, value: MandatesCreateBankRequestScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn sequence_type(mut self, value: MandatesCreateBankRequestSequenceType) -> Self {
        self.sequence_type = Some(value);
        self
    }

    pub fn signature_date(mut self, value: NaiveDate) -> Self {
        self.signature_date = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn debtor_name(mut self, value: impl Into<String>) -> Self {
        self.debtor_name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MandatesCreateBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](MandatesCreateBankRequestBuilder::partner_id)
    /// - [`signature_date`](MandatesCreateBankRequestBuilder::signature_date)
    pub fn build(self) -> Result<MandatesCreateBankRequest, BuildError> {
        Ok(MandatesCreateBankRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            iban: self.iban,
            bic: self.bic,
            bank_code: self.bank_code,
            bank_account_number: self.bank_account_number,
            account_type: self.account_type,
            scheme: self.scheme,
            sequence_type: self.sequence_type,
            signature_date: self
                .signature_date
                .ok_or_else(|| BuildError::missing_field("signature_date"))?,
            reference: self.reference,
            debtor_name: self.debtor_name,
            notes: self.notes,
        })
    }
}
