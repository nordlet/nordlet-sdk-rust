pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkV7MGenerateDeclarationsResponseDeclarationItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PlJpkV7MGenerateDeclarationsResponseDeclarationItem {
    pub fn builder() -> PlJpkV7MGenerateDeclarationsResponseDeclarationItemBuilder {
        <PlJpkV7MGenerateDeclarationsResponseDeclarationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkV7MGenerateDeclarationsResponseDeclarationItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PlJpkV7MGenerateDeclarationsResponseDeclarationItemBuilder {
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

    /// Consumes the builder and constructs a [`PlJpkV7MGenerateDeclarationsResponseDeclarationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PlJpkV7MGenerateDeclarationsResponseDeclarationItemBuilder::field)
    /// - [`label`](PlJpkV7MGenerateDeclarationsResponseDeclarationItemBuilder::label)
    /// - [`value`](PlJpkV7MGenerateDeclarationsResponseDeclarationItemBuilder::value)
    pub fn build(self) -> Result<PlJpkV7MGenerateDeclarationsResponseDeclarationItem, BuildError> {
        Ok(PlJpkV7MGenerateDeclarationsResponseDeclarationItem {
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
