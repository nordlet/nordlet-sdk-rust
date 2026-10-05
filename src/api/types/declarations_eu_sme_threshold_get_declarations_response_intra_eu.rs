pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdGetDeclarationsResponseIntraEu {
    #[serde(default)]
    pub trigger: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "acquisitionsFromMemberStates")]
    #[serde(default)]
    pub acquisitions_from_member_states: String,
    #[serde(rename = "servicesToMemberStates")]
    #[serde(default)]
    pub services_to_member_states: String,
    #[serde(default)]
    pub total: String,
    pub status: EuSmeThresholdGetDeclarationsResponseIntraEuStatus,
    #[serde(default)]
    pub note: String,
}

impl EuSmeThresholdGetDeclarationsResponseIntraEu {
    pub fn builder() -> EuSmeThresholdGetDeclarationsResponseIntraEuBuilder {
        <EuSmeThresholdGetDeclarationsResponseIntraEuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdGetDeclarationsResponseIntraEuBuilder {
    trigger: Option<String>,
    currency: Option<String>,
    acquisitions_from_member_states: Option<String>,
    services_to_member_states: Option<String>,
    total: Option<String>,
    status: Option<EuSmeThresholdGetDeclarationsResponseIntraEuStatus>,
    note: Option<String>,
}

impl EuSmeThresholdGetDeclarationsResponseIntraEuBuilder {
    pub fn trigger(mut self, value: impl Into<String>) -> Self {
        self.trigger = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn acquisitions_from_member_states(mut self, value: impl Into<String>) -> Self {
        self.acquisitions_from_member_states = Some(value.into());
        self
    }

    pub fn services_to_member_states(mut self, value: impl Into<String>) -> Self {
        self.services_to_member_states = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn status(mut self, value: EuSmeThresholdGetDeclarationsResponseIntraEuStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuSmeThresholdGetDeclarationsResponseIntraEu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trigger`](EuSmeThresholdGetDeclarationsResponseIntraEuBuilder::trigger)
    /// - [`currency`](EuSmeThresholdGetDeclarationsResponseIntraEuBuilder::currency)
    /// - [`acquisitions_from_member_states`](EuSmeThresholdGetDeclarationsResponseIntraEuBuilder::acquisitions_from_member_states)
    /// - [`services_to_member_states`](EuSmeThresholdGetDeclarationsResponseIntraEuBuilder::services_to_member_states)
    /// - [`total`](EuSmeThresholdGetDeclarationsResponseIntraEuBuilder::total)
    /// - [`status`](EuSmeThresholdGetDeclarationsResponseIntraEuBuilder::status)
    /// - [`note`](EuSmeThresholdGetDeclarationsResponseIntraEuBuilder::note)
    pub fn build(self) -> Result<EuSmeThresholdGetDeclarationsResponseIntraEu, BuildError> {
        Ok(EuSmeThresholdGetDeclarationsResponseIntraEu {
            trigger: self
                .trigger
                .ok_or_else(|| BuildError::missing_field("trigger"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            acquisitions_from_member_states: self
                .acquisitions_from_member_states
                .ok_or_else(|| BuildError::missing_field("acquisitions_from_member_states"))?,
            services_to_member_states: self
                .services_to_member_states
                .ok_or_else(|| BuildError::missing_field("services_to_member_states"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            note: self.note.ok_or_else(|| BuildError::missing_field("note"))?,
        })
    }
}
