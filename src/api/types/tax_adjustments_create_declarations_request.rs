pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsCreateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    pub kind: TaxAdjustmentsCreateDeclarationsRequestKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl TaxAdjustmentsCreateDeclarationsRequest {
    pub fn builder() -> TaxAdjustmentsCreateDeclarationsRequestBuilder {
        <TaxAdjustmentsCreateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsCreateDeclarationsRequestBuilder {
    year: Option<i64>,
    kind: Option<TaxAdjustmentsCreateDeclarationsRequestKind>,
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl TaxAdjustmentsCreateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn kind(mut self, value: TaxAdjustmentsCreateDeclarationsRequestKind) -> Self {
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

    /// Consumes the builder and constructs a [`TaxAdjustmentsCreateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](TaxAdjustmentsCreateDeclarationsRequestBuilder::year)
    /// - [`kind`](TaxAdjustmentsCreateDeclarationsRequestBuilder::kind)
    /// - [`amount`](TaxAdjustmentsCreateDeclarationsRequestBuilder::amount)
    /// - [`description`](TaxAdjustmentsCreateDeclarationsRequestBuilder::description)
    pub fn build(self) -> Result<TaxAdjustmentsCreateDeclarationsRequest, BuildError> {
        Ok(TaxAdjustmentsCreateDeclarationsRequest {
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
