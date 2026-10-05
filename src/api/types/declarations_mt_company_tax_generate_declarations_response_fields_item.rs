pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtCompanyTaxGenerateDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl MtCompanyTaxGenerateDeclarationsResponseFieldsItem {
    pub fn builder() -> MtCompanyTaxGenerateDeclarationsResponseFieldsItemBuilder {
        <MtCompanyTaxGenerateDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtCompanyTaxGenerateDeclarationsResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl MtCompanyTaxGenerateDeclarationsResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`MtCompanyTaxGenerateDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](MtCompanyTaxGenerateDeclarationsResponseFieldsItemBuilder::field)
    /// - [`label`](MtCompanyTaxGenerateDeclarationsResponseFieldsItemBuilder::label)
    /// - [`value`](MtCompanyTaxGenerateDeclarationsResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<MtCompanyTaxGenerateDeclarationsResponseFieldsItem, BuildError> {
        Ok(MtCompanyTaxGenerateDeclarationsResponseFieldsItem {
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
