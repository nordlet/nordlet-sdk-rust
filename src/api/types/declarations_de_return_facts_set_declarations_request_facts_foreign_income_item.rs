pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub kind: DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemKind,
    #[serde(default)]
    pub income: String,
}

impl DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemBuilder {
        <DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemBuilder {
    country_code: Option<String>,
    kind: Option<DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemKind>,
    income: Option<String>,
}

impl DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn income(mut self, value: impl Into<String>) -> Self {
        self.income = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemBuilder::country_code)
    /// - [`kind`](DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemBuilder::kind)
    /// - [`income`](DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItemBuilder::income)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsRequestFactsForeignIncomeItem {
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
