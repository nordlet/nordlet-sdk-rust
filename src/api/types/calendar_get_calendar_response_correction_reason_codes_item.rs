pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCalendarResponseCorrectionReasonCodesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
}

impl GetCalendarResponseCorrectionReasonCodesItem {
    pub fn builder() -> GetCalendarResponseCorrectionReasonCodesItemBuilder {
        <GetCalendarResponseCorrectionReasonCodesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCalendarResponseCorrectionReasonCodesItemBuilder {
    code: Option<String>,
    label: Option<String>,
}

impl GetCalendarResponseCorrectionReasonCodesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCalendarResponseCorrectionReasonCodesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GetCalendarResponseCorrectionReasonCodesItemBuilder::code)
    /// - [`label`](GetCalendarResponseCorrectionReasonCodesItemBuilder::label)
    pub fn build(self) -> Result<GetCalendarResponseCorrectionReasonCodesItem, BuildError> {
        Ok(GetCalendarResponseCorrectionReasonCodesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
