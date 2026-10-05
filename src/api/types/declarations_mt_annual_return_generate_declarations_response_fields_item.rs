pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtAnnualReturnGenerateDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl MtAnnualReturnGenerateDeclarationsResponseFieldsItem {
    pub fn builder() -> MtAnnualReturnGenerateDeclarationsResponseFieldsItemBuilder {
        <MtAnnualReturnGenerateDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtAnnualReturnGenerateDeclarationsResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl MtAnnualReturnGenerateDeclarationsResponseFieldsItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MtAnnualReturnGenerateDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](MtAnnualReturnGenerateDeclarationsResponseFieldsItemBuilder::field)
    /// - [`label`](MtAnnualReturnGenerateDeclarationsResponseFieldsItemBuilder::label)
    /// - [`value`](MtAnnualReturnGenerateDeclarationsResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<MtAnnualReturnGenerateDeclarationsResponseFieldsItem, BuildError> {
        Ok(MtAnnualReturnGenerateDeclarationsResponseFieldsItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
