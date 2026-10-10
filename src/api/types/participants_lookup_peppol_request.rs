pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ParticipantsLookupPeppolRequest {
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "participantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
}

impl ParticipantsLookupPeppolRequest {
    pub fn builder() -> ParticipantsLookupPeppolRequestBuilder {
        <ParticipantsLookupPeppolRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ParticipantsLookupPeppolRequestBuilder {
    partner_id: Option<String>,
    participant_id: Option<String>,
}

impl ParticipantsLookupPeppolRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn participant_id(mut self, value: impl Into<String>) -> Self {
        self.participant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ParticipantsLookupPeppolRequest`].
    pub fn build(self) -> Result<ParticipantsLookupPeppolRequest, BuildError> {
        Ok(ParticipantsLookupPeppolRequest {
            partner_id: self.partner_id,
            participant_id: self.participant_id,
        })
    }
}
