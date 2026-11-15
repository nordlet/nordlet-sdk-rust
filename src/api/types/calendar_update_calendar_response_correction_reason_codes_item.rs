pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCalendarResponseCorrectionReasonCodesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
}

impl UpdateCalendarResponseCorrectionReasonCodesItem {
    pub fn builder() -> UpdateCalendarResponseCorrectionReasonCodesItemBuilder {
        <UpdateCalendarResponseCorrectionReasonCodesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCalendarResponseCorrectionReasonCodesItemBuilder {
    code: Option<String>,
    label: Option<String>,
}

impl UpdateCalendarResponseCorrectionReasonCodesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCalendarResponseCorrectionReasonCodesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UpdateCalendarResponseCorrectionReasonCodesItemBuilder::code)
    /// - [`label`](UpdateCalendarResponseCorrectionReasonCodesItemBuilder::label)
    pub fn build(self) -> Result<UpdateCalendarResponseCorrectionReasonCodesItem, BuildError> {
        Ok(UpdateCalendarResponseCorrectionReasonCodesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
