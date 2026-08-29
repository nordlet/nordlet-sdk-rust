pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankMandatesCancelResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub reference: String,
    pub scheme: PostV1BankMandatesCancelResponseScheme,
    #[serde(rename = "sequenceType")]
    pub sequence_type: PostV1BankMandatesCancelResponseSequenceType,
    pub status: PostV1BankMandatesCancelResponseStatus,
    #[serde(rename = "debtorName")]
    #[serde(default)]
    pub debtor_name: String,
    #[serde(default)]
    pub iban: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(rename = "signatureDate")]
    #[serde(default)]
    pub signature_date: String,
    #[serde(rename = "collectionsCount")]
    #[serde(default)]
    pub collections_count: i64,
    #[serde(rename = "lastCollectionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_collection_date: Option<String>,
    #[serde(rename = "expiresOn")]
    #[serde(default)]
    pub expires_on: String,
    #[serde(rename = "cancelledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1BankMandatesCancelResponse {
    pub fn builder() -> PostV1BankMandatesCancelResponseBuilder {
        <PostV1BankMandatesCancelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMandatesCancelResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    reference: Option<String>,
    scheme: Option<PostV1BankMandatesCancelResponseScheme>,
    sequence_type: Option<PostV1BankMandatesCancelResponseSequenceType>,
    status: Option<PostV1BankMandatesCancelResponseStatus>,
    debtor_name: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    signature_date: Option<String>,
    collections_count: Option<i64>,
    last_collection_date: Option<String>,
    expires_on: Option<String>,
    cancelled_at: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1BankMandatesCancelResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: PostV1BankMandatesCancelResponseScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn sequence_type(mut self, value: PostV1BankMandatesCancelResponseSequenceType) -> Self {
        self.sequence_type = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1BankMandatesCancelResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn debtor_name(mut self, value: impl Into<String>) -> Self {
        self.debtor_name = Some(value.into());
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

    pub fn signature_date(mut self, value: impl Into<String>) -> Self {
        self.signature_date = Some(value.into());
        self
    }

    pub fn collections_count(mut self, value: i64) -> Self {
        self.collections_count = Some(value);
        self
    }

    pub fn last_collection_date(mut self, value: impl Into<String>) -> Self {
        self.last_collection_date = Some(value.into());
        self
    }

    pub fn expires_on(mut self, value: impl Into<String>) -> Self {
        self.expires_on = Some(value.into());
        self
    }

    pub fn cancelled_at(mut self, value: impl Into<String>) -> Self {
        self.cancelled_at = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1BankMandatesCancelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMandatesCancelResponseBuilder::id)
    /// - [`partner_id`](PostV1BankMandatesCancelResponseBuilder::partner_id)
    /// - [`reference`](PostV1BankMandatesCancelResponseBuilder::reference)
    /// - [`scheme`](PostV1BankMandatesCancelResponseBuilder::scheme)
    /// - [`sequence_type`](PostV1BankMandatesCancelResponseBuilder::sequence_type)
    /// - [`status`](PostV1BankMandatesCancelResponseBuilder::status)
    /// - [`debtor_name`](PostV1BankMandatesCancelResponseBuilder::debtor_name)
    /// - [`iban`](PostV1BankMandatesCancelResponseBuilder::iban)
    /// - [`signature_date`](PostV1BankMandatesCancelResponseBuilder::signature_date)
    /// - [`collections_count`](PostV1BankMandatesCancelResponseBuilder::collections_count)
    /// - [`expires_on`](PostV1BankMandatesCancelResponseBuilder::expires_on)
    /// - [`created_at`](PostV1BankMandatesCancelResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1BankMandatesCancelResponse, BuildError> {
        Ok(PostV1BankMandatesCancelResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            sequence_type: self
                .sequence_type
                .ok_or_else(|| BuildError::missing_field("sequence_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            debtor_name: self
                .debtor_name
                .ok_or_else(|| BuildError::missing_field("debtor_name"))?,
            iban: self.iban.ok_or_else(|| BuildError::missing_field("iban"))?,
            bic: self.bic,
            signature_date: self
                .signature_date
                .ok_or_else(|| BuildError::missing_field("signature_date"))?,
            collections_count: self
                .collections_count
                .ok_or_else(|| BuildError::missing_field("collections_count"))?,
            last_collection_date: self.last_collection_date,
            expires_on: self
                .expires_on
                .ok_or_else(|| BuildError::missing_field("expires_on"))?,
            cancelled_at: self.cancelled_at,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
