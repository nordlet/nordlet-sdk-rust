pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatProrataSetDeclarationsRequest {
    #[serde(rename = "fiscalYear")]
    #[serde(default)]
    pub fiscal_year: i64,
    #[serde(default)]
    pub percent: String,
}

impl VatProrataSetDeclarationsRequest {
    pub fn builder() -> VatProrataSetDeclarationsRequestBuilder {
        <VatProrataSetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatProrataSetDeclarationsRequestBuilder {
    fiscal_year: Option<i64>,
    percent: Option<String>,
}

impl VatProrataSetDeclarationsRequestBuilder {
    pub fn fiscal_year(mut self, value: i64) -> Self {
        self.fiscal_year = Some(value);
        self
    }

    pub fn percent(mut self, value: impl Into<String>) -> Self {
        self.percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatProrataSetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fiscal_year`](VatProrataSetDeclarationsRequestBuilder::fiscal_year)
    /// - [`percent`](VatProrataSetDeclarationsRequestBuilder::percent)
    pub fn build(self) -> Result<VatProrataSetDeclarationsRequest, BuildError> {
        Ok(VatProrataSetDeclarationsRequest {
            fiscal_year: self
                .fiscal_year
                .ok_or_else(|| BuildError::missing_field("fiscal_year"))?,
            percent: self
                .percent
                .ok_or_else(|| BuildError::missing_field("percent"))?,
        })
    }
}
