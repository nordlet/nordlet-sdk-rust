pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDac7PreviewDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub system: String,
    #[serde(rename = "sendsDirectly")]
    #[serde(default)]
    pub sends_directly: bool,
    #[serde(rename = "messageTypeIndic")]
    #[serde(default)]
    pub message_type_indic: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub sellers: Vec<EuDac7PreviewDeclarationsResponseSellersItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl EuDac7PreviewDeclarationsResponse {
    pub fn builder() -> EuDac7PreviewDeclarationsResponseBuilder {
        <EuDac7PreviewDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDac7PreviewDeclarationsResponseBuilder {
    year: Option<i64>,
    country: Option<String>,
    system: Option<String>,
    sends_directly: Option<bool>,
    message_type_indic: Option<String>,
    currency: Option<String>,
    sellers: Option<Vec<EuDac7PreviewDeclarationsResponseSellersItem>>,
    warnings: Option<Vec<String>>,
    source: Option<String>,
}

impl EuDac7PreviewDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn sends_directly(mut self, value: bool) -> Self {
        self.sends_directly = Some(value);
        self
    }

    pub fn message_type_indic(mut self, value: impl Into<String>) -> Self {
        self.message_type_indic = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn sellers(mut self, value: Vec<EuDac7PreviewDeclarationsResponseSellersItem>) -> Self {
        self.sellers = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuDac7PreviewDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuDac7PreviewDeclarationsResponseBuilder::year)
    /// - [`country`](EuDac7PreviewDeclarationsResponseBuilder::country)
    /// - [`system`](EuDac7PreviewDeclarationsResponseBuilder::system)
    /// - [`sends_directly`](EuDac7PreviewDeclarationsResponseBuilder::sends_directly)
    /// - [`message_type_indic`](EuDac7PreviewDeclarationsResponseBuilder::message_type_indic)
    /// - [`currency`](EuDac7PreviewDeclarationsResponseBuilder::currency)
    /// - [`sellers`](EuDac7PreviewDeclarationsResponseBuilder::sellers)
    /// - [`warnings`](EuDac7PreviewDeclarationsResponseBuilder::warnings)
    /// - [`source`](EuDac7PreviewDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<EuDac7PreviewDeclarationsResponse, BuildError> {
        Ok(EuDac7PreviewDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            sends_directly: self
                .sends_directly
                .ok_or_else(|| BuildError::missing_field("sends_directly"))?,
            message_type_indic: self
                .message_type_indic
                .ok_or_else(|| BuildError::missing_field("message_type_indic"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            sellers: self
                .sellers
                .ok_or_else(|| BuildError::missing_field("sellers"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
