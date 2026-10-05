pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxPaymentsListDeclarationsResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub tax: String,
    #[serde(default)]
    pub year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    pub kind: TaxPaymentsListDeclarationsResponseRowsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "paidOn")]
    #[serde(default)]
    pub paid_on: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default)]
    pub description: String,
}

impl TaxPaymentsListDeclarationsResponseRowsItem {
    pub fn builder() -> TaxPaymentsListDeclarationsResponseRowsItemBuilder {
        <TaxPaymentsListDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxPaymentsListDeclarationsResponseRowsItemBuilder {
    id: Option<String>,
    tax: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    kind: Option<TaxPaymentsListDeclarationsResponseRowsItemKind>,
    amount: Option<String>,
    paid_on: Option<String>,
    reference: Option<String>,
    description: Option<String>,
}

impl TaxPaymentsListDeclarationsResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tax(mut self, value: impl Into<String>) -> Self {
        self.tax = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn kind(mut self, value: TaxPaymentsListDeclarationsResponseRowsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn paid_on(mut self, value: impl Into<String>) -> Self {
        self.paid_on = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxPaymentsListDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxPaymentsListDeclarationsResponseRowsItemBuilder::id)
    /// - [`tax`](TaxPaymentsListDeclarationsResponseRowsItemBuilder::tax)
    /// - [`year`](TaxPaymentsListDeclarationsResponseRowsItemBuilder::year)
    /// - [`kind`](TaxPaymentsListDeclarationsResponseRowsItemBuilder::kind)
    /// - [`amount`](TaxPaymentsListDeclarationsResponseRowsItemBuilder::amount)
    /// - [`paid_on`](TaxPaymentsListDeclarationsResponseRowsItemBuilder::paid_on)
    /// - [`description`](TaxPaymentsListDeclarationsResponseRowsItemBuilder::description)
    pub fn build(self) -> Result<TaxPaymentsListDeclarationsResponseRowsItem, BuildError> {
        Ok(TaxPaymentsListDeclarationsResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self.month,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            paid_on: self
                .paid_on
                .ok_or_else(|| BuildError::missing_field("paid_on"))?,
            reference: self.reference,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
