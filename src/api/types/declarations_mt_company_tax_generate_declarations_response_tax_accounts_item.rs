pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub amount: String,
}

impl MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem {
    pub fn builder() -> MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItemBuilder {
        <MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    amount: Option<String>,
}

impl MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItemBuilder {
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

    /// Consumes the builder and constructs a [`MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItemBuilder::code)
    /// - [`label`](MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItemBuilder::label)
    /// - [`amount`](MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem, BuildError> {
        Ok(MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem {
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
