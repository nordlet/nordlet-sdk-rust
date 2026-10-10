pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ParticipantsLookupPeppolResponse {
    #[serde(rename = "participantId")]
    #[serde(default)]
    pub participant_id: String,
    #[serde(default)]
    pub registered: bool,
    #[serde(rename = "smpUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_url: Option<String>,
    #[serde(rename = "accessPointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_url: Option<String>,
    #[serde(rename = "acceptsInvoice")]
    #[serde(default)]
    pub accepts_invoice: bool,
    #[serde(rename = "acceptsCreditNote")]
    #[serde(default)]
    pub accepts_credit_note: bool,
    #[serde(rename = "acceptsCii")]
    #[serde(default)]
    pub accepts_cii: bool,
}

impl ParticipantsLookupPeppolResponse {
    pub fn builder() -> ParticipantsLookupPeppolResponseBuilder {
        <ParticipantsLookupPeppolResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ParticipantsLookupPeppolResponseBuilder {
    participant_id: Option<String>,
    registered: Option<bool>,
    smp_url: Option<String>,
    access_point_url: Option<String>,
    accepts_invoice: Option<bool>,
    accepts_credit_note: Option<bool>,
    accepts_cii: Option<bool>,
}

impl ParticipantsLookupPeppolResponseBuilder {
    pub fn participant_id(mut self, value: impl Into<String>) -> Self {
        self.participant_id = Some(value.into());
        self
    }

    pub fn registered(mut self, value: bool) -> Self {
        self.registered = Some(value);
        self
    }

    pub fn smp_url(mut self, value: impl Into<String>) -> Self {
        self.smp_url = Some(value.into());
        self
    }

    pub fn access_point_url(mut self, value: impl Into<String>) -> Self {
        self.access_point_url = Some(value.into());
        self
    }

    pub fn accepts_invoice(mut self, value: bool) -> Self {
        self.accepts_invoice = Some(value);
        self
    }

    pub fn accepts_credit_note(mut self, value: bool) -> Self {
        self.accepts_credit_note = Some(value);
        self
    }

    pub fn accepts_cii(mut self, value: bool) -> Self {
        self.accepts_cii = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ParticipantsLookupPeppolResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`participant_id`](ParticipantsLookupPeppolResponseBuilder::participant_id)
    /// - [`registered`](ParticipantsLookupPeppolResponseBuilder::registered)
    /// - [`accepts_invoice`](ParticipantsLookupPeppolResponseBuilder::accepts_invoice)
    /// - [`accepts_credit_note`](ParticipantsLookupPeppolResponseBuilder::accepts_credit_note)
    /// - [`accepts_cii`](ParticipantsLookupPeppolResponseBuilder::accepts_cii)
    pub fn build(self) -> Result<ParticipantsLookupPeppolResponse, BuildError> {
        Ok(ParticipantsLookupPeppolResponse {
            participant_id: self
                .participant_id
                .ok_or_else(|| BuildError::missing_field("participant_id"))?,
            registered: self
                .registered
                .ok_or_else(|| BuildError::missing_field("registered"))?,
            smp_url: self.smp_url,
            access_point_url: self.access_point_url,
            accepts_invoice: self
                .accepts_invoice
                .ok_or_else(|| BuildError::missing_field("accepts_invoice"))?,
            accepts_credit_note: self
                .accepts_credit_note
                .ok_or_else(|| BuildError::missing_field("accepts_credit_note"))?,
            accepts_cii: self
                .accepts_cii
                .ok_or_else(|| BuildError::missing_field("accepts_cii"))?,
        })
    }
}
