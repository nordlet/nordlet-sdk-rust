pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtGpm313ComputeDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl LtGpm313ComputeDeclarationsResponseFieldsItem {
    pub fn builder() -> LtGpm313ComputeDeclarationsResponseFieldsItemBuilder {
        <LtGpm313ComputeDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm313ComputeDeclarationsResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl LtGpm313ComputeDeclarationsResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`LtGpm313ComputeDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](LtGpm313ComputeDeclarationsResponseFieldsItemBuilder::field)
    /// - [`label`](LtGpm313ComputeDeclarationsResponseFieldsItemBuilder::label)
    /// - [`value`](LtGpm313ComputeDeclarationsResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<LtGpm313ComputeDeclarationsResponseFieldsItem, BuildError> {
        Ok(LtGpm313ComputeDeclarationsResponseFieldsItem {
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
