pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuUnionTurnoverGetDeclarationsResponsePreviousYear {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl EuUnionTurnoverGetDeclarationsResponsePreviousYear {
    pub fn builder() -> EuUnionTurnoverGetDeclarationsResponsePreviousYearBuilder {
        <EuUnionTurnoverGetDeclarationsResponsePreviousYearBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuUnionTurnoverGetDeclarationsResponsePreviousYearBuilder {
    year: Option<i64>,
    amount: Option<String>,
    documents: Option<i64>,
}

impl EuUnionTurnoverGetDeclarationsResponsePreviousYearBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuUnionTurnoverGetDeclarationsResponsePreviousYear`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuUnionTurnoverGetDeclarationsResponsePreviousYearBuilder::year)
    /// - [`amount`](EuUnionTurnoverGetDeclarationsResponsePreviousYearBuilder::amount)
    /// - [`documents`](EuUnionTurnoverGetDeclarationsResponsePreviousYearBuilder::documents)
    pub fn build(self) -> Result<EuUnionTurnoverGetDeclarationsResponsePreviousYear, BuildError> {
        Ok(EuUnionTurnoverGetDeclarationsResponsePreviousYear {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
