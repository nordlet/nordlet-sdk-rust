pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtPln204ComputeDeclarationsResponseAnnexSItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl LtPln204ComputeDeclarationsResponseAnnexSItem {
    pub fn builder() -> LtPln204ComputeDeclarationsResponseAnnexSItemBuilder {
        <LtPln204ComputeDeclarationsResponseAnnexSItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204ComputeDeclarationsResponseAnnexSItemBuilder {
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl LtPln204ComputeDeclarationsResponseAnnexSItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtPln204ComputeDeclarationsResponseAnnexSItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LtPln204ComputeDeclarationsResponseAnnexSItemBuilder::code)
    /// - [`amount`](LtPln204ComputeDeclarationsResponseAnnexSItemBuilder::amount)
    /// - [`description`](LtPln204ComputeDeclarationsResponseAnnexSItemBuilder::description)
    pub fn build(self) -> Result<LtPln204ComputeDeclarationsResponseAnnexSItem, BuildError> {
        Ok(LtPln204ComputeDeclarationsResponseAnnexSItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
