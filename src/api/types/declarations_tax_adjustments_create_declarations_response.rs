pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsCreateDeclarationsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    pub kind: TaxAdjustmentsCreateDeclarationsResponseKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl TaxAdjustmentsCreateDeclarationsResponse {
    pub fn builder() -> TaxAdjustmentsCreateDeclarationsResponseBuilder {
        <TaxAdjustmentsCreateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsCreateDeclarationsResponseBuilder {
    id: Option<String>,
    year: Option<i64>,
    kind: Option<TaxAdjustmentsCreateDeclarationsResponseKind>,
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl TaxAdjustmentsCreateDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn kind(mut self, value: TaxAdjustmentsCreateDeclarationsResponseKind) -> Self {
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

    /// Consumes the builder and constructs a [`TaxAdjustmentsCreateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxAdjustmentsCreateDeclarationsResponseBuilder::id)
    /// - [`year`](TaxAdjustmentsCreateDeclarationsResponseBuilder::year)
    /// - [`kind`](TaxAdjustmentsCreateDeclarationsResponseBuilder::kind)
    /// - [`amount`](TaxAdjustmentsCreateDeclarationsResponseBuilder::amount)
    /// - [`description`](TaxAdjustmentsCreateDeclarationsResponseBuilder::description)
    pub fn build(self) -> Result<TaxAdjustmentsCreateDeclarationsResponse, BuildError> {
        Ok(TaxAdjustmentsCreateDeclarationsResponse {
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
