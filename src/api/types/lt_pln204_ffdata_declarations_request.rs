pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtPln204FfdataDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl LtPln204FfdataDeclarationsRequest {
    pub fn builder() -> LtPln204FfdataDeclarationsRequestBuilder {
        <LtPln204FfdataDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204FfdataDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl LtPln204FfdataDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtPln204FfdataDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtPln204FfdataDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<LtPln204FfdataDeclarationsRequest, BuildError> {
        Ok(LtPln204FfdataDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
