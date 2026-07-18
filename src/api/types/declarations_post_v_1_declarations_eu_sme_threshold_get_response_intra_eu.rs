pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdGetResponseIntraEu {
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
    pub status: PostV1DeclarationsEuSmeThresholdGetResponseIntraEuStatus,
    #[serde(default)]
    pub note: String,
}

impl PostV1DeclarationsEuSmeThresholdGetResponseIntraEu {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder {
        <PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder {
    trigger: Option<String>,
    currency: Option<String>,
    acquisitions_from_member_states: Option<String>,
    services_to_member_states: Option<String>,
    total: Option<String>,
    status: Option<PostV1DeclarationsEuSmeThresholdGetResponseIntraEuStatus>,
    note: Option<String>,
}

impl PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder {
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

    pub fn status(
        mut self,
        value: PostV1DeclarationsEuSmeThresholdGetResponseIntraEuStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdGetResponseIntraEu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trigger`](PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder::trigger)
    /// - [`currency`](PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder::currency)
    /// - [`acquisitions_from_member_states`](PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder::acquisitions_from_member_states)
    /// - [`services_to_member_states`](PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder::services_to_member_states)
    /// - [`total`](PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder::total)
    /// - [`status`](PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder::status)
    /// - [`note`](PostV1DeclarationsEuSmeThresholdGetResponseIntraEuBuilder::note)
    pub fn build(self) -> Result<PostV1DeclarationsEuSmeThresholdGetResponseIntraEu, BuildError> {
        Ok(PostV1DeclarationsEuSmeThresholdGetResponseIntraEu {
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
