pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCalendarResponseCorrection {
    #[serde(default)]
    pub required: Vec<CreateCalendarResponseCorrectionRequiredItem>,
    #[serde(default)]
    pub optional: Vec<CreateCalendarResponseCorrectionOptionalItem>,
    #[serde(rename = "reasonCodes")]
    #[serde(default)]
    pub reason_codes: Vec<CreateCalendarResponseCorrectionReasonCodesItem>,
    #[serde(default)]
    pub carry: bool,
}

impl CreateCalendarResponseCorrection {
    pub fn builder() -> CreateCalendarResponseCorrectionBuilder {
        <CreateCalendarResponseCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCalendarResponseCorrectionBuilder {
    required: Option<Vec<CreateCalendarResponseCorrectionRequiredItem>>,
    optional: Option<Vec<CreateCalendarResponseCorrectionOptionalItem>>,
    reason_codes: Option<Vec<CreateCalendarResponseCorrectionReasonCodesItem>>,
    carry: Option<bool>,
}

impl CreateCalendarResponseCorrectionBuilder {
    pub fn required(mut self, value: Vec<CreateCalendarResponseCorrectionRequiredItem>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn optional(mut self, value: Vec<CreateCalendarResponseCorrectionOptionalItem>) -> Self {
        self.optional = Some(value);
        self
    }

    pub fn reason_codes(
        mut self,
        value: Vec<CreateCalendarResponseCorrectionReasonCodesItem>,
    ) -> Self {
        self.reason_codes = Some(value);
        self
    }

    pub fn carry(mut self, value: bool) -> Self {
        self.carry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCalendarResponseCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](CreateCalendarResponseCorrectionBuilder::required)
    /// - [`optional`](CreateCalendarResponseCorrectionBuilder::optional)
    /// - [`reason_codes`](CreateCalendarResponseCorrectionBuilder::reason_codes)
    /// - [`carry`](CreateCalendarResponseCorrectionBuilder::carry)
    pub fn build(self) -> Result<CreateCalendarResponseCorrection, BuildError> {
        Ok(CreateCalendarResponseCorrection {
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
