pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsGetDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl AnnualAccountsGetDeclarationsRequest {
    pub fn builder() -> AnnualAccountsGetDeclarationsRequestBuilder {
        <AnnualAccountsGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsGetDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl AnnualAccountsGetDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AnnualAccountsGetDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<AnnualAccountsGetDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsGetDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
