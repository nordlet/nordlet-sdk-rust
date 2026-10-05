pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0600ComputeDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl LtFr0600ComputeDeclarationsResponseFieldsItem {
    pub fn builder() -> LtFr0600ComputeDeclarationsResponseFieldsItemBuilder {
        <LtFr0600ComputeDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0600ComputeDeclarationsResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl LtFr0600ComputeDeclarationsResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`LtFr0600ComputeDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](LtFr0600ComputeDeclarationsResponseFieldsItemBuilder::field)
    /// - [`label`](LtFr0600ComputeDeclarationsResponseFieldsItemBuilder::label)
    /// - [`value`](LtFr0600ComputeDeclarationsResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<LtFr0600ComputeDeclarationsResponseFieldsItem, BuildError> {
        Ok(LtFr0600ComputeDeclarationsResponseFieldsItem {
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
