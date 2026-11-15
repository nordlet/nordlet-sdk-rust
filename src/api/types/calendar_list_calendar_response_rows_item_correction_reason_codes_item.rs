pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCalendarResponseRowsItemCorrectionReasonCodesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
}

impl ListCalendarResponseRowsItemCorrectionReasonCodesItem {
    pub fn builder() -> ListCalendarResponseRowsItemCorrectionReasonCodesItemBuilder {
        <ListCalendarResponseRowsItemCorrectionReasonCodesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCalendarResponseRowsItemCorrectionReasonCodesItemBuilder {
    code: Option<String>,
    label: Option<String>,
}

impl ListCalendarResponseRowsItemCorrectionReasonCodesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCalendarResponseRowsItemCorrectionReasonCodesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ListCalendarResponseRowsItemCorrectionReasonCodesItemBuilder::code)
    /// - [`label`](ListCalendarResponseRowsItemCorrectionReasonCodesItemBuilder::label)
    pub fn build(
        self,
    ) -> Result<ListCalendarResponseRowsItemCorrectionReasonCodesItem, BuildError> {
        Ok(ListCalendarResponseRowsItemCorrectionReasonCodesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
