pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem {
    pub fn builder() -> EuSmeCrossBorderReportComputeDeclarationsResponseRowsItemBuilder {
        <EuSmeCrossBorderReportComputeDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeCrossBorderReportComputeDeclarationsResponseRowsItemBuilder {
    country_code: Option<String>,
    amount: Option<String>,
    documents: Option<i64>,
}

impl EuSmeCrossBorderReportComputeDeclarationsResponseRowsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
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

    /// Consumes the builder and constructs a [`EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuSmeCrossBorderReportComputeDeclarationsResponseRowsItemBuilder::country_code)
    /// - [`amount`](EuSmeCrossBorderReportComputeDeclarationsResponseRowsItemBuilder::amount)
    /// - [`documents`](EuSmeCrossBorderReportComputeDeclarationsResponseRowsItemBuilder::documents)
    pub fn build(
        self,
    ) -> Result<EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem, BuildError> {
        Ok(EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
