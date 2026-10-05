pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIntrastatObligationDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl LtIntrastatObligationDeclarationsRequest {
    pub fn builder() -> LtIntrastatObligationDeclarationsRequestBuilder {
        <LtIntrastatObligationDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatObligationDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl LtIntrastatObligationDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatObligationDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtIntrastatObligationDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<LtIntrastatObligationDeclarationsRequest, BuildError> {
        Ok(LtIntrastatObligationDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
