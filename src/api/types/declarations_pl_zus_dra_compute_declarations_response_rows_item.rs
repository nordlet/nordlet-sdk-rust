pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlZusDraComputeDeclarationsResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub insured: String,
    #[serde(default)]
    pub payer: String,
    #[serde(default)]
    pub total: String,
}

impl PlZusDraComputeDeclarationsResponseRowsItem {
    pub fn builder() -> PlZusDraComputeDeclarationsResponseRowsItemBuilder {
        <PlZusDraComputeDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlZusDraComputeDeclarationsResponseRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    insured: Option<String>,
    payer: Option<String>,
    total: Option<String>,
}

impl PlZusDraComputeDeclarationsResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn insured(mut self, value: impl Into<String>) -> Self {
        self.insured = Some(value.into());
        self
    }

    pub fn payer(mut self, value: impl Into<String>) -> Self {
        self.payer = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlZusDraComputeDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PlZusDraComputeDeclarationsResponseRowsItemBuilder::code)
    /// - [`label`](PlZusDraComputeDeclarationsResponseRowsItemBuilder::label)
    /// - [`insured`](PlZusDraComputeDeclarationsResponseRowsItemBuilder::insured)
    /// - [`payer`](PlZusDraComputeDeclarationsResponseRowsItemBuilder::payer)
    /// - [`total`](PlZusDraComputeDeclarationsResponseRowsItemBuilder::total)
    pub fn build(self) -> Result<PlZusDraComputeDeclarationsResponseRowsItem, BuildError> {
        Ok(PlZusDraComputeDeclarationsResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            insured: self
                .insured
                .ok_or_else(|| BuildError::missing_field("insured"))?,
            payer: self
                .payer
                .ok_or_else(|| BuildError::missing_field("payer"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
