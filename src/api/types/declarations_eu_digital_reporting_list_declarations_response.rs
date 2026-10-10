pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDigitalReportingListDeclarationsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "appliesFrom")]
    #[serde(default)]
    pub applies_from: String,
    #[serde(rename = "reportTo")]
    #[serde(default)]
    pub report_to: String,
    #[serde(default)]
    pub transactions: Vec<EuDigitalReportingListDeclarationsResponseTransactionsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl EuDigitalReportingListDeclarationsResponse {
    pub fn builder() -> EuDigitalReportingListDeclarationsResponseBuilder {
        <EuDigitalReportingListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDigitalReportingListDeclarationsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    applies_from: Option<String>,
    report_to: Option<String>,
    transactions: Option<Vec<EuDigitalReportingListDeclarationsResponseTransactionsItem>>,
    warnings: Option<Vec<String>>,
    source: Option<String>,
}

impl EuDigitalReportingListDeclarationsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn applies_from(mut self, value: impl Into<String>) -> Self {
        self.applies_from = Some(value.into());
        self
    }

    pub fn report_to(mut self, value: impl Into<String>) -> Self {
        self.report_to = Some(value.into());
        self
    }

    pub fn transactions(
        mut self,
        value: Vec<EuDigitalReportingListDeclarationsResponseTransactionsItem>,
    ) -> Self {
        self.transactions = Some(value);
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

    /// Consumes the builder and constructs a [`EuDigitalReportingListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](EuDigitalReportingListDeclarationsResponseBuilder::from_date)
    /// - [`to_date`](EuDigitalReportingListDeclarationsResponseBuilder::to_date)
    /// - [`applies_from`](EuDigitalReportingListDeclarationsResponseBuilder::applies_from)
    /// - [`report_to`](EuDigitalReportingListDeclarationsResponseBuilder::report_to)
    /// - [`transactions`](EuDigitalReportingListDeclarationsResponseBuilder::transactions)
    /// - [`warnings`](EuDigitalReportingListDeclarationsResponseBuilder::warnings)
    /// - [`source`](EuDigitalReportingListDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<EuDigitalReportingListDeclarationsResponse, BuildError> {
        Ok(EuDigitalReportingListDeclarationsResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            applies_from: self
                .applies_from
                .ok_or_else(|| BuildError::missing_field("applies_from"))?,
            report_to: self
                .report_to
                .ok_or_else(|| BuildError::missing_field("report_to"))?,
            transactions: self
                .transactions
                .ok_or_else(|| BuildError::missing_field("transactions"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
