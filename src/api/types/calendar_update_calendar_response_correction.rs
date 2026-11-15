pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCalendarResponseCorrection {
    #[serde(default)]
    pub required: Vec<UpdateCalendarResponseCorrectionRequiredItem>,
    #[serde(default)]
    pub optional: Vec<UpdateCalendarResponseCorrectionOptionalItem>,
    #[serde(rename = "reasonCodes")]
    #[serde(default)]
    pub reason_codes: Vec<UpdateCalendarResponseCorrectionReasonCodesItem>,
    #[serde(default)]
    pub carry: bool,
}

impl UpdateCalendarResponseCorrection {
    pub fn builder() -> UpdateCalendarResponseCorrectionBuilder {
        <UpdateCalendarResponseCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCalendarResponseCorrectionBuilder {
    required: Option<Vec<UpdateCalendarResponseCorrectionRequiredItem>>,
    optional: Option<Vec<UpdateCalendarResponseCorrectionOptionalItem>>,
    reason_codes: Option<Vec<UpdateCalendarResponseCorrectionReasonCodesItem>>,
    carry: Option<bool>,
}

impl UpdateCalendarResponseCorrectionBuilder {
    pub fn required(mut self, value: Vec<UpdateCalendarResponseCorrectionRequiredItem>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn optional(mut self, value: Vec<UpdateCalendarResponseCorrectionOptionalItem>) -> Self {
        self.optional = Some(value);
        self
    }

    pub fn reason_codes(
        mut self,
        value: Vec<UpdateCalendarResponseCorrectionReasonCodesItem>,
    ) -> Self {
        self.reason_codes = Some(value);
        self
    }

    pub fn carry(mut self, value: bool) -> Self {
        self.carry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCalendarResponseCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](UpdateCalendarResponseCorrectionBuilder::required)
    /// - [`optional`](UpdateCalendarResponseCorrectionBuilder::optional)
    /// - [`reason_codes`](UpdateCalendarResponseCorrectionBuilder::reason_codes)
    /// - [`carry`](UpdateCalendarResponseCorrectionBuilder::carry)
    pub fn build(self) -> Result<UpdateCalendarResponseCorrection, BuildError> {
        Ok(UpdateCalendarResponseCorrection {
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
