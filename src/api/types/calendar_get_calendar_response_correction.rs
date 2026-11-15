pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCalendarResponseCorrection {
    #[serde(default)]
    pub required: Vec<GetCalendarResponseCorrectionRequiredItem>,
    #[serde(default)]
    pub optional: Vec<GetCalendarResponseCorrectionOptionalItem>,
    #[serde(rename = "reasonCodes")]
    #[serde(default)]
    pub reason_codes: Vec<GetCalendarResponseCorrectionReasonCodesItem>,
    #[serde(default)]
    pub carry: bool,
}

impl GetCalendarResponseCorrection {
    pub fn builder() -> GetCalendarResponseCorrectionBuilder {
        <GetCalendarResponseCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCalendarResponseCorrectionBuilder {
    required: Option<Vec<GetCalendarResponseCorrectionRequiredItem>>,
    optional: Option<Vec<GetCalendarResponseCorrectionOptionalItem>>,
    reason_codes: Option<Vec<GetCalendarResponseCorrectionReasonCodesItem>>,
    carry: Option<bool>,
}

impl GetCalendarResponseCorrectionBuilder {
    pub fn required(mut self, value: Vec<GetCalendarResponseCorrectionRequiredItem>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn optional(mut self, value: Vec<GetCalendarResponseCorrectionOptionalItem>) -> Self {
        self.optional = Some(value);
        self
    }

    pub fn reason_codes(
        mut self,
        value: Vec<GetCalendarResponseCorrectionReasonCodesItem>,
    ) -> Self {
        self.reason_codes = Some(value);
        self
    }

    pub fn carry(mut self, value: bool) -> Self {
        self.carry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCalendarResponseCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](GetCalendarResponseCorrectionBuilder::required)
    /// - [`optional`](GetCalendarResponseCorrectionBuilder::optional)
    /// - [`reason_codes`](GetCalendarResponseCorrectionBuilder::reason_codes)
    /// - [`carry`](GetCalendarResponseCorrectionBuilder::carry)
    pub fn build(self) -> Result<GetCalendarResponseCorrection, BuildError> {
        Ok(GetCalendarResponseCorrection {
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            optional: self
                .optional
                .ok_or_else(|| BuildError::missing_field("optional"))?,
            reason_codes: self
                .reason_codes
                .ok_or_else(|| BuildError::missing_field("reason_codes"))?,
            carry: self
                .carry
                .ok_or_else(|| BuildError::missing_field("carry"))?,
        })
    }
}
