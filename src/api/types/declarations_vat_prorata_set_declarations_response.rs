pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatProrataSetDeclarationsResponse {
    #[serde(rename = "fiscalYear")]
    #[serde(default)]
    pub fiscal_year: i64,
    #[serde(default)]
    pub percent: String,
}

impl VatProrataSetDeclarationsResponse {
    pub fn builder() -> VatProrataSetDeclarationsResponseBuilder {
        <VatProrataSetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatProrataSetDeclarationsResponseBuilder {
    fiscal_year: Option<i64>,
    percent: Option<String>,
}

impl VatProrataSetDeclarationsResponseBuilder {
    pub fn fiscal_year(mut self, value: i64) -> Self {
        self.fiscal_year = Some(value);
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatProrataSetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fiscal_year`](VatProrataSetDeclarationsResponseBuilder::fiscal_year)
    /// - [`percent`](VatProrataSetDeclarationsResponseBuilder::percent)
    pub fn build(self) -> Result<VatProrataSetDeclarationsResponse, BuildError> {
        Ok(VatProrataSetDeclarationsResponse {
            fiscal_year: self
                .fiscal_year
                .ok_or_else(|| BuildError::missing_field("fiscal_year"))?,
            percent: self
                .percent
                .ok_or_else(|| BuildError::missing_field("percent"))?,
        })
    }
}
