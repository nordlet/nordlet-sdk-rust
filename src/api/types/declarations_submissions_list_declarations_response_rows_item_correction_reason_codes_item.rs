pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
}

impl SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem {
    pub fn builder() -> SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItemBuilder
    {
        <SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItemBuilder {
    code: Option<String>,
    label: Option<String>,
}

impl SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItemBuilder::code)
    /// - [`label`](SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItemBuilder::label)
    pub fn build(
        self,
    ) -> Result<SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem, BuildError>
    {
        Ok(
            SubmissionsListDeclarationsResponseRowsItemCorrectionReasonCodesItem {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
            },
        )
    }
}
