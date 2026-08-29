pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMandatesCreateRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub iban: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<PostV1BankMandatesCreateRequestScheme>,
    #[serde(rename = "sequenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_type: Option<PostV1BankMandatesCreateRequestSequenceType>,
    #[serde(rename = "signatureDate")]
    #[serde(default)]
    pub signature_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "debtorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debtor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1BankMandatesCreateRequest {
    pub fn builder() -> PostV1BankMandatesCreateRequestBuilder {
        <PostV1BankMandatesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMandatesCreateRequestBuilder {
    partner_id: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    scheme: Option<PostV1BankMandatesCreateRequestScheme>,
    sequence_type: Option<PostV1BankMandatesCreateRequestSequenceType>,
    signature_date: Option<String>,
    reference: Option<String>,
    debtor_name: Option<String>,
    notes: Option<String>,
}

impl PostV1BankMandatesCreateRequestBuilder {
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

    pub fn scheme(mut self, value: PostV1BankMandatesCreateRequestScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn sequence_type(mut self, value: PostV1BankMandatesCreateRequestSequenceType) -> Self {
        self.sequence_type = Some(value);
        self
    }

    pub fn signature_date(mut self, value: impl Into<String>) -> Self {
        self.signature_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1BankMandatesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1BankMandatesCreateRequestBuilder::partner_id)
    /// - [`iban`](PostV1BankMandatesCreateRequestBuilder::iban)
    /// - [`signature_date`](PostV1BankMandatesCreateRequestBuilder::signature_date)
    pub fn build(self) -> Result<PostV1BankMandatesCreateRequest, BuildError> {
        Ok(PostV1BankMandatesCreateRequest {
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
