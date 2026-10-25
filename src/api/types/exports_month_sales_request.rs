pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExportsMonthSalesRequest {
    #[serde(default)]
    pub month: String,
    pub format: ExportsMonthSalesRequestFormat,
}

impl ExportsMonthSalesRequest {
    pub fn builder() -> ExportsMonthSalesRequestBuilder {
        <ExportsMonthSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportsMonthSalesRequestBuilder {
    month: Option<String>,
    format: Option<ExportsMonthSalesRequestFormat>,
}

impl ExportsMonthSalesRequestBuilder {
    pub fn month(mut self, value: impl Into<String>) -> Self {
        self.month = Some(value.into());
        self
    }

    pub fn format(mut self, value: ExportsMonthSalesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportsMonthSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`month`](ExportsMonthSalesRequestBuilder::month)
    /// - [`format`](ExportsMonthSalesRequestBuilder::format)
    pub fn build(self) -> Result<ExportsMonthSalesRequest, BuildError> {
        Ok(ExportsMonthSalesRequest {
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
        })
    }
}
