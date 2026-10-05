pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsListDeclarationsResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    pub kind: TaxAdjustmentsListDeclarationsResponseRowsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl TaxAdjustmentsListDeclarationsResponseRowsItem {
    pub fn builder() -> TaxAdjustmentsListDeclarationsResponseRowsItemBuilder {
        <TaxAdjustmentsListDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsListDeclarationsResponseRowsItemBuilder {
    id: Option<String>,
    year: Option<i64>,
    kind: Option<TaxAdjustmentsListDeclarationsResponseRowsItemKind>,
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl TaxAdjustmentsListDeclarationsResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn kind(mut self, value: TaxAdjustmentsListDeclarationsResponseRowsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`TaxAdjustmentsListDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxAdjustmentsListDeclarationsResponseRowsItemBuilder::id)
    /// - [`year`](TaxAdjustmentsListDeclarationsResponseRowsItemBuilder::year)
    /// - [`kind`](TaxAdjustmentsListDeclarationsResponseRowsItemBuilder::kind)
    /// - [`amount`](TaxAdjustmentsListDeclarationsResponseRowsItemBuilder::amount)
    /// - [`description`](TaxAdjustmentsListDeclarationsResponseRowsItemBuilder::description)
    pub fn build(self) -> Result<TaxAdjustmentsListDeclarationsResponseRowsItem, BuildError> {
        Ok(TaxAdjustmentsListDeclarationsResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            code: self.code,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
