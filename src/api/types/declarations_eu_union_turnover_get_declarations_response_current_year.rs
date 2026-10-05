pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuUnionTurnoverGetDeclarationsResponseCurrentYear {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl EuUnionTurnoverGetDeclarationsResponseCurrentYear {
    pub fn builder() -> EuUnionTurnoverGetDeclarationsResponseCurrentYearBuilder {
        <EuUnionTurnoverGetDeclarationsResponseCurrentYearBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuUnionTurnoverGetDeclarationsResponseCurrentYearBuilder {
    year: Option<i64>,
    amount: Option<String>,
    documents: Option<i64>,
}

impl EuUnionTurnoverGetDeclarationsResponseCurrentYearBuilder {
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

    /// Consumes the builder and constructs a [`EuUnionTurnoverGetDeclarationsResponseCurrentYear`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuUnionTurnoverGetDeclarationsResponseCurrentYearBuilder::year)
    /// - [`amount`](EuUnionTurnoverGetDeclarationsResponseCurrentYearBuilder::amount)
    /// - [`documents`](EuUnionTurnoverGetDeclarationsResponseCurrentYearBuilder::documents)
    pub fn build(self) -> Result<EuUnionTurnoverGetDeclarationsResponseCurrentYear, BuildError> {
        Ok(EuUnionTurnoverGetDeclarationsResponseCurrentYear {
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
