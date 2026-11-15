pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCalendarResponseCorrectionReasonCodesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
}

impl CreateCalendarResponseCorrectionReasonCodesItem {
    pub fn builder() -> CreateCalendarResponseCorrectionReasonCodesItemBuilder {
        <CreateCalendarResponseCorrectionReasonCodesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCalendarResponseCorrectionReasonCodesItemBuilder {
    code: Option<String>,
    label: Option<String>,
}

impl CreateCalendarResponseCorrectionReasonCodesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCalendarResponseCorrectionReasonCodesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CreateCalendarResponseCorrectionReasonCodesItemBuilder::code)
    /// - [`label`](CreateCalendarResponseCorrectionReasonCodesItemBuilder::label)
    pub fn build(self) -> Result<CreateCalendarResponseCorrectionReasonCodesItem, BuildError> {
        Ok(CreateCalendarResponseCorrectionReasonCodesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
