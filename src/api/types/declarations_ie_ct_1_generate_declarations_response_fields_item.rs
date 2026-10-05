pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeCt1GenerateDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl IeCt1GenerateDeclarationsResponseFieldsItem {
    pub fn builder() -> IeCt1GenerateDeclarationsResponseFieldsItemBuilder {
        <IeCt1GenerateDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeCt1GenerateDeclarationsResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl IeCt1GenerateDeclarationsResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`IeCt1GenerateDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](IeCt1GenerateDeclarationsResponseFieldsItemBuilder::field)
    /// - [`label`](IeCt1GenerateDeclarationsResponseFieldsItemBuilder::label)
    /// - [`value`](IeCt1GenerateDeclarationsResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<IeCt1GenerateDeclarationsResponseFieldsItem, BuildError> {
        Ok(IeCt1GenerateDeclarationsResponseFieldsItem {
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
