pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlIntrastatGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub flow: PlIntrastatGenerateDeclarationsRequestFlow,
    #[serde(rename = "transactionNature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_nature: Option<String>,
}

impl PlIntrastatGenerateDeclarationsRequest {
    pub fn builder() -> PlIntrastatGenerateDeclarationsRequestBuilder {
        <PlIntrastatGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlIntrastatGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    flow: Option<PlIntrastatGenerateDeclarationsRequestFlow>,
    transaction_nature: Option<String>,
}

impl PlIntrastatGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn flow(mut self, value: PlIntrastatGenerateDeclarationsRequestFlow) -> Self {
        self.flow = Some(value);
        self
    }

    pub fn transaction_nature(mut self, value: impl Into<String>) -> Self {
        self.transaction_nature = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlIntrastatGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlIntrastatGenerateDeclarationsRequestBuilder::year)
    /// - [`month`](PlIntrastatGenerateDeclarationsRequestBuilder::month)
    /// - [`flow`](PlIntrastatGenerateDeclarationsRequestBuilder::flow)
    pub fn build(self) -> Result<PlIntrastatGenerateDeclarationsRequest, BuildError> {
        Ok(PlIntrastatGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            flow: self.flow.ok_or_else(|| BuildError::missing_field("flow"))?,
            transaction_nature: self.transaction_nature,
        })
    }
}
