pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtPln204ComputeDeclarationsResponseAnnexZItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl LtPln204ComputeDeclarationsResponseAnnexZItem {
    pub fn builder() -> LtPln204ComputeDeclarationsResponseAnnexZItemBuilder {
        <LtPln204ComputeDeclarationsResponseAnnexZItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204ComputeDeclarationsResponseAnnexZItemBuilder {
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl LtPln204ComputeDeclarationsResponseAnnexZItemBuilder {
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

    /// Consumes the builder and constructs a [`LtPln204ComputeDeclarationsResponseAnnexZItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LtPln204ComputeDeclarationsResponseAnnexZItemBuilder::code)
    /// - [`amount`](LtPln204ComputeDeclarationsResponseAnnexZItemBuilder::amount)
    /// - [`description`](LtPln204ComputeDeclarationsResponseAnnexZItemBuilder::description)
    pub fn build(self) -> Result<LtPln204ComputeDeclarationsResponseAnnexZItem, BuildError> {
        Ok(LtPln204ComputeDeclarationsResponseAnnexZItem {
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
