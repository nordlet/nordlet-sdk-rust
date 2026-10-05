pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CyHe32GenerateDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl CyHe32GenerateDeclarationsResponseFieldsItem {
    pub fn builder() -> CyHe32GenerateDeclarationsResponseFieldsItemBuilder {
        <CyHe32GenerateDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CyHe32GenerateDeclarationsResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl CyHe32GenerateDeclarationsResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`CyHe32GenerateDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CyHe32GenerateDeclarationsResponseFieldsItemBuilder::field)
    /// - [`label`](CyHe32GenerateDeclarationsResponseFieldsItemBuilder::label)
    /// - [`value`](CyHe32GenerateDeclarationsResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<CyHe32GenerateDeclarationsResponseFieldsItem, BuildError> {
        Ok(CyHe32GenerateDeclarationsResponseFieldsItem {
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
