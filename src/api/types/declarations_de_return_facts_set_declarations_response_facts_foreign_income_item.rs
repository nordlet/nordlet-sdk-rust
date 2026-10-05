pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub kind: DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemKind,
    #[serde(default)]
    pub income: String,
}

impl DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemBuilder {
        <DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemBuilder {
    country_code: Option<String>,
    kind: Option<DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemKind>,
    income: Option<String>,
}

impl DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn income(mut self, value: impl Into<String>) -> Self {
        self.income = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemBuilder::country_code)
    /// - [`kind`](DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemBuilder::kind)
    /// - [`income`](DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItemBuilder::income)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsResponseFactsForeignIncomeItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            income: self
                .income
                .ok_or_else(|| BuildError::missing_field("income"))?,
        })
    }
}
