pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtPln204ComputeDeclarationsResponseLinesItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl LtPln204ComputeDeclarationsResponseLinesItem {
    pub fn builder() -> LtPln204ComputeDeclarationsResponseLinesItemBuilder {
        <LtPln204ComputeDeclarationsResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204ComputeDeclarationsResponseLinesItemBuilder {
    key: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl LtPln204ComputeDeclarationsResponseLinesItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
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

    /// Consumes the builder and constructs a [`LtPln204ComputeDeclarationsResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](LtPln204ComputeDeclarationsResponseLinesItemBuilder::key)
    /// - [`label`](LtPln204ComputeDeclarationsResponseLinesItemBuilder::label)
    /// - [`value`](LtPln204ComputeDeclarationsResponseLinesItemBuilder::value)
    pub fn build(self) -> Result<LtPln204ComputeDeclarationsResponseLinesItem, BuildError> {
        Ok(LtPln204ComputeDeclarationsResponseLinesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
