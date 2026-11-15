pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitCalendarResponseCorrectionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "reasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(rename = "authorityReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_reference: Option<String>,
    #[serde(rename = "discoveredOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_on: Option<String>,
}

impl SubmitCalendarResponseCorrectionInput {
    pub fn builder() -> SubmitCalendarResponseCorrectionInputBuilder {
        <SubmitCalendarResponseCorrectionInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitCalendarResponseCorrectionInputBuilder {
    reason: Option<String>,
    reason_code: Option<String>,
    authority_reference: Option<String>,
    discovered_on: Option<String>,
}

impl SubmitCalendarResponseCorrectionInputBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.reason_code = Some(value.into());
        self
    }

    pub fn authority_reference(mut self, value: impl Into<String>) -> Self {
        self.authority_reference = Some(value.into());
        self
    }

    pub fn discovered_on(mut self, value: impl Into<String>) -> Self {
        self.discovered_on = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitCalendarResponseCorrectionInput`].
    pub fn build(self) -> Result<SubmitCalendarResponseCorrectionInput, BuildError> {
        Ok(SubmitCalendarResponseCorrectionInput {
            reason: self.reason,
            reason_code: self.reason_code,
            authority_reference: self.authority_reference,
            discovered_on: self.discovered_on,
        })
    }
}
