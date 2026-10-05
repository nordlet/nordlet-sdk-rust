pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DkSelskabsskatGenerateDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl DkSelskabsskatGenerateDeclarationsResponseFieldsItem {
    pub fn builder() -> DkSelskabsskatGenerateDeclarationsResponseFieldsItemBuilder {
        <DkSelskabsskatGenerateDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DkSelskabsskatGenerateDeclarationsResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl DkSelskabsskatGenerateDeclarationsResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`DkSelskabsskatGenerateDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DkSelskabsskatGenerateDeclarationsResponseFieldsItemBuilder::field)
    /// - [`label`](DkSelskabsskatGenerateDeclarationsResponseFieldsItemBuilder::label)
    /// - [`value`](DkSelskabsskatGenerateDeclarationsResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<DkSelskabsskatGenerateDeclarationsResponseFieldsItem, BuildError> {
        Ok(DkSelskabsskatGenerateDeclarationsResponseFieldsItem {
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
