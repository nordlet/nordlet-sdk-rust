pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCalendarResponseRowsItemCorrection {
    #[serde(default)]
    pub required: Vec<ListCalendarResponseRowsItemCorrectionRequiredItem>,
    #[serde(default)]
    pub optional: Vec<ListCalendarResponseRowsItemCorrectionOptionalItem>,
    #[serde(rename = "reasonCodes")]
    #[serde(default)]
    pub reason_codes: Vec<ListCalendarResponseRowsItemCorrectionReasonCodesItem>,
    #[serde(default)]
    pub carry: bool,
}

impl ListCalendarResponseRowsItemCorrection {
    pub fn builder() -> ListCalendarResponseRowsItemCorrectionBuilder {
        <ListCalendarResponseRowsItemCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCalendarResponseRowsItemCorrectionBuilder {
    required: Option<Vec<ListCalendarResponseRowsItemCorrectionRequiredItem>>,
    optional: Option<Vec<ListCalendarResponseRowsItemCorrectionOptionalItem>>,
    reason_codes: Option<Vec<ListCalendarResponseRowsItemCorrectionReasonCodesItem>>,
    carry: Option<bool>,
}

impl ListCalendarResponseRowsItemCorrectionBuilder {
    pub fn required(
        mut self,
        value: Vec<ListCalendarResponseRowsItemCorrectionRequiredItem>,
    ) -> Self {
        self.required = Some(value);
        self
    }

    pub fn optional(
        mut self,
        value: Vec<ListCalendarResponseRowsItemCorrectionOptionalItem>,
    ) -> Self {
        self.optional = Some(value);
        self
    }

    pub fn reason_codes(
        mut self,
        value: Vec<ListCalendarResponseRowsItemCorrectionReasonCodesItem>,
    ) -> Self {
        self.reason_codes = Some(value);
        self
    }

    pub fn carry(mut self, value: bool) -> Self {
        self.carry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCalendarResponseRowsItemCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](ListCalendarResponseRowsItemCorrectionBuilder::required)
    /// - [`optional`](ListCalendarResponseRowsItemCorrectionBuilder::optional)
    /// - [`reason_codes`](ListCalendarResponseRowsItemCorrectionBuilder::reason_codes)
    /// - [`carry`](ListCalendarResponseRowsItemCorrectionBuilder::carry)
    pub fn build(self) -> Result<ListCalendarResponseRowsItemCorrection, BuildError> {
        Ok(ListCalendarResponseRowsItemCorrection {
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
