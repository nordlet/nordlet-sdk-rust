pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatProrataGetDeclarationsRequest {
    #[serde(rename = "fiscalYear")]
    #[serde(default)]
    pub fiscal_year: i64,
}

impl VatProrataGetDeclarationsRequest {
    pub fn builder() -> VatProrataGetDeclarationsRequestBuilder {
        <VatProrataGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatProrataGetDeclarationsRequestBuilder {
    fiscal_year: Option<i64>,
}

impl VatProrataGetDeclarationsRequestBuilder {
    pub fn fiscal_year(mut self, value: i64) -> Self {
        self.fiscal_year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatProrataGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fiscal_year`](VatProrataGetDeclarationsRequestBuilder::fiscal_year)
    pub fn build(self) -> Result<VatProrataGetDeclarationsRequest, BuildError> {
        Ok(VatProrataGetDeclarationsRequest {
            fiscal_year: self
                .fiscal_year
                .ok_or_else(|| BuildError::missing_field("fiscal_year"))?,
        })
    }
}
