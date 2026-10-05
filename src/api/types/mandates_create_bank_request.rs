pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MandatesCreateBankRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub iban: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
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
    /// - [`iban`](MandatesCreateBankRequestBuilder::iban)
    /// - [`signature_date`](MandatesCreateBankRequestBuilder::signature_date)
    pub fn build(self) -> Result<MandatesCreateBankRequest, BuildError> {
        Ok(MandatesCreateBankRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            iban: self.iban.ok_or_else(|| BuildError::missing_field("iban"))?,
            bic: self.bic,
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
