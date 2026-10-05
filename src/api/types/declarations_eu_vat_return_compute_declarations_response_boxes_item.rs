pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuVatReturnComputeDeclarationsResponseBoxesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub amount: String,
}

impl EuVatReturnComputeDeclarationsResponseBoxesItem {
    pub fn builder() -> EuVatReturnComputeDeclarationsResponseBoxesItemBuilder {
        <EuVatReturnComputeDeclarationsResponseBoxesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatReturnComputeDeclarationsResponseBoxesItemBuilder {
    code: Option<String>,
    label: Option<String>,
    amount: Option<String>,
}

impl EuVatReturnComputeDeclarationsResponseBoxesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuVatReturnComputeDeclarationsResponseBoxesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](EuVatReturnComputeDeclarationsResponseBoxesItemBuilder::code)
    /// - [`label`](EuVatReturnComputeDeclarationsResponseBoxesItemBuilder::label)
    /// - [`amount`](EuVatReturnComputeDeclarationsResponseBoxesItemBuilder::amount)
    pub fn build(self) -> Result<EuVatReturnComputeDeclarationsResponseBoxesItem, BuildError> {
        Ok(EuVatReturnComputeDeclarationsResponseBoxesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
