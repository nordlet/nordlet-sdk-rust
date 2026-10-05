pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MandatesUpdateBankResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub reference: String,
    pub scheme: MandatesUpdateBankResponseScheme,
    #[serde(rename = "sequenceType")]
    pub sequence_type: MandatesUpdateBankResponseSequenceType,
    pub status: MandatesUpdateBankResponseStatus,
    #[serde(rename = "debtorName")]
    #[serde(default)]
    pub debtor_name: String,
    #[serde(default)]
    pub iban: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(rename = "signatureDate")]
    #[serde(default)]
    pub signature_date: NaiveDate,
    #[serde(rename = "collectionsCount")]
    #[serde(default)]
    pub collections_count: i64,
    #[serde(rename = "lastCollectionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_collection_date: Option<NaiveDate>,
    #[serde(rename = "expiresOn")]
    #[serde(default)]
    pub expires_on: String,
    #[serde(rename = "cancelledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub cancelled_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl MandatesUpdateBankResponse {
    pub fn builder() -> MandatesUpdateBankResponseBuilder {
        <MandatesUpdateBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesUpdateBankResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    reference: Option<String>,
    scheme: Option<MandatesUpdateBankResponseScheme>,
    sequence_type: Option<MandatesUpdateBankResponseSequenceType>,
    status: Option<MandatesUpdateBankResponseStatus>,
    debtor_name: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    signature_date: Option<NaiveDate>,
    collections_count: Option<i64>,
    last_collection_date: Option<NaiveDate>,
    expires_on: Option<String>,
    cancelled_at: Option<DateTime<FixedOffset>>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl MandatesUpdateBankResponseBuilder {
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

    pub fn scheme(mut self, value: MandatesUpdateBankResponseScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn sequence_type(mut self, value: MandatesUpdateBankResponseSequenceType) -> Self {
        self.sequence_type = Some(value);
        self
    }

    pub fn status(mut self, value: MandatesUpdateBankResponseStatus) -> Self {
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

    pub fn signature_date(mut self, value: NaiveDate) -> Self {
        self.signature_date = Some(value);
        self
    }

    pub fn collections_count(mut self, value: i64) -> Self {
        self.collections_count = Some(value);
        self
    }

    pub fn last_collection_date(mut self, value: NaiveDate) -> Self {
        self.last_collection_date = Some(value);
        self
    }

    pub fn expires_on(mut self, value: impl Into<String>) -> Self {
        self.expires_on = Some(value.into());
        self
    }

    pub fn cancelled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.cancelled_at = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MandatesUpdateBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MandatesUpdateBankResponseBuilder::id)
    /// - [`partner_id`](MandatesUpdateBankResponseBuilder::partner_id)
    /// - [`reference`](MandatesUpdateBankResponseBuilder::reference)
    /// - [`scheme`](MandatesUpdateBankResponseBuilder::scheme)
    /// - [`sequence_type`](MandatesUpdateBankResponseBuilder::sequence_type)
    /// - [`status`](MandatesUpdateBankResponseBuilder::status)
    /// - [`debtor_name`](MandatesUpdateBankResponseBuilder::debtor_name)
    /// - [`iban`](MandatesUpdateBankResponseBuilder::iban)
    /// - [`signature_date`](MandatesUpdateBankResponseBuilder::signature_date)
    /// - [`collections_count`](MandatesUpdateBankResponseBuilder::collections_count)
    /// - [`expires_on`](MandatesUpdateBankResponseBuilder::expires_on)
    /// - [`created_at`](MandatesUpdateBankResponseBuilder::created_at)
    pub fn build(self) -> Result<MandatesUpdateBankResponse, BuildError> {
        Ok(MandatesUpdateBankResponse {
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
