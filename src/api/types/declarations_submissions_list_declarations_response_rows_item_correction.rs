pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsListDeclarationsResponseRowsItemCorrection {
    #[serde(default)]
    pub required: Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionRequiredItem>,
    #[serde(default)]
    pub optional: Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionOptionalItem>,
    #[serde(rename = "reasonCodes")]
    #[serde(default)]
    pub reason_codes: Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem>,
    #[serde(default)]
    pub carry: bool,
}

impl SubmissionsListDeclarationsResponseRowsItemCorrection {
    pub fn builder() -> SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder {
        <SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder {
    required: Option<Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionRequiredItem>>,
    optional: Option<Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionOptionalItem>>,
    reason_codes: Option<Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem>>,
    carry: Option<bool>,
}

impl SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder {
    pub fn required(
        mut self,
        value: Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionRequiredItem>,
    ) -> Self {
        self.required = Some(value);
        self
    }

    pub fn optional(
        mut self,
        value: Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionOptionalItem>,
    ) -> Self {
        self.optional = Some(value);
        self
    }

    pub fn reason_codes(
        mut self,
        value: Vec<SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem>,
    ) -> Self {
        self.reason_codes = Some(value);
        self
    }

    pub fn carry(mut self, value: bool) -> Self {
        self.carry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsListDeclarationsResponseRowsItemCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder::required)
    /// - [`optional`](SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder::optional)
    /// - [`reason_codes`](SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder::reason_codes)
    /// - [`carry`](SubmissionsListDeclarationsResponseRowsItemCorrectionBuilder::carry)
    pub fn build(
        self,
    ) -> Result<SubmissionsListDeclarationsResponseRowsItemCorrection, BuildError> {
        Ok(SubmissionsListDeclarationsResponseRowsItemCorrection {
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
