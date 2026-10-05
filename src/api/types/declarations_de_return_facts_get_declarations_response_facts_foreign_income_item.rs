pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub kind: DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemKind,
    #[serde(default)]
    pub income: String,
}

impl DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItem {
    pub fn builder() -> DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemBuilder {
        <DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemBuilder {
    country_code: Option<String>,
    kind: Option<DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemKind>,
    income: Option<String>,
}

impl DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn income(mut self, value: impl Into<String>) -> Self {
        self.income = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemBuilder::country_code)
    /// - [`kind`](DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemBuilder::kind)
    /// - [`income`](DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItemBuilder::income)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItem, BuildError> {
        Ok(DeReturnFactsGetDeclarationsResponseFactsForeignIncomeItem {
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
