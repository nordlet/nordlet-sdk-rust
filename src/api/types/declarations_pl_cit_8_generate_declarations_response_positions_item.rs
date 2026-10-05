pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlCit8GenerateDeclarationsResponsePositionsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PlCit8GenerateDeclarationsResponsePositionsItem {
    pub fn builder() -> PlCit8GenerateDeclarationsResponsePositionsItemBuilder {
        <PlCit8GenerateDeclarationsResponsePositionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlCit8GenerateDeclarationsResponsePositionsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PlCit8GenerateDeclarationsResponsePositionsItemBuilder {
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

    /// Consumes the builder and constructs a [`PlCit8GenerateDeclarationsResponsePositionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PlCit8GenerateDeclarationsResponsePositionsItemBuilder::field)
    /// - [`label`](PlCit8GenerateDeclarationsResponsePositionsItemBuilder::label)
    /// - [`value`](PlCit8GenerateDeclarationsResponsePositionsItemBuilder::value)
    pub fn build(self) -> Result<PlCit8GenerateDeclarationsResponsePositionsItem, BuildError> {
        Ok(PlCit8GenerateDeclarationsResponsePositionsItem {
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
