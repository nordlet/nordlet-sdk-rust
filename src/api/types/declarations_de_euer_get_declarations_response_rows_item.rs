pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeEuerGetDeclarationsResponseRowsItem {
    pub anlage: DeEuerGetDeclarationsResponseRowsItemAnlage,
    #[serde(default)]
    pub zeile: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kennzahl: Option<String>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl DeEuerGetDeclarationsResponseRowsItem {
    pub fn builder() -> DeEuerGetDeclarationsResponseRowsItemBuilder {
        <DeEuerGetDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeEuerGetDeclarationsResponseRowsItemBuilder {
    anlage: Option<DeEuerGetDeclarationsResponseRowsItemAnlage>,
    zeile: Option<i64>,
    kennzahl: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl DeEuerGetDeclarationsResponseRowsItemBuilder {
    pub fn anlage(mut self, value: DeEuerGetDeclarationsResponseRowsItemAnlage) -> Self {
        self.anlage = Some(value);
        self
    }

    pub fn zeile(mut self, value: i64) -> Self {
        self.zeile = Some(value);
        self
    }

    pub fn kennzahl(mut self, value: impl Into<String>) -> Self {
        self.kennzahl = Some(value.into());
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

    /// Consumes the builder and constructs a [`DeEuerGetDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`anlage`](DeEuerGetDeclarationsResponseRowsItemBuilder::anlage)
    /// - [`zeile`](DeEuerGetDeclarationsResponseRowsItemBuilder::zeile)
    /// - [`label`](DeEuerGetDeclarationsResponseRowsItemBuilder::label)
    /// - [`value`](DeEuerGetDeclarationsResponseRowsItemBuilder::value)
    pub fn build(self) -> Result<DeEuerGetDeclarationsResponseRowsItem, BuildError> {
        Ok(DeEuerGetDeclarationsResponseRowsItem {
            anlage: self
                .anlage
                .ok_or_else(|| BuildError::missing_field("anlage"))?,
            zeile: self
                .zeile
                .ok_or_else(|| BuildError::missing_field("zeile"))?,
            kennzahl: self.kennzahl,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
