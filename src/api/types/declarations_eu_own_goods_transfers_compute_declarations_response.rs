pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuOwnGoodsTransfersComputeDeclarationsResponse {
    #[serde(rename = "periodYear")]
    #[serde(default)]
    pub period_year: i64,
    #[serde(rename = "periodMonth")]
    #[serde(default)]
    pub period_month: i64,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: NaiveDate,
    #[serde(rename = "memberStateOfIdentification")]
    #[serde(default)]
    pub member_state_of_identification: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub rows: Vec<EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub total: String,
    #[serde(default)]
    pub transfers: Vec<EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl EuOwnGoodsTransfersComputeDeclarationsResponse {
    pub fn builder() -> EuOwnGoodsTransfersComputeDeclarationsResponseBuilder {
        <EuOwnGoodsTransfersComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOwnGoodsTransfersComputeDeclarationsResponseBuilder {
    period_year: Option<i64>,
    period_month: Option<i64>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    member_state_of_identification: Option<String>,
    currency: Option<String>,
    rows: Option<Vec<EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem>>,
    total: Option<String>,
    transfers: Option<Vec<EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem>>,
    warnings: Option<Vec<String>>,
    source: Option<String>,
}

impl EuOwnGoodsTransfersComputeDeclarationsResponseBuilder {
    pub fn period_year(mut self, value: i64) -> Self {
        self.period_year = Some(value);
        self
    }

    pub fn period_month(mut self, value: i64) -> Self {
        self.period_month = Some(value);
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn member_state_of_identification(mut self, value: impl Into<String>) -> Self {
        self.member_state_of_identification = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn transfers(
        mut self,
        value: Vec<EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem>,
    ) -> Self {
        self.transfers = Some(value);
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

    /// Consumes the builder and constructs a [`EuOwnGoodsTransfersComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_year`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::period_year)
    /// - [`period_month`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::period_month)
    /// - [`from_date`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::from_date)
    /// - [`to_date`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::to_date)
    /// - [`due_date`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::due_date)
    /// - [`member_state_of_identification`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::member_state_of_identification)
    /// - [`currency`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::currency)
    /// - [`rows`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::rows)
    /// - [`total`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::total)
    /// - [`transfers`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::transfers)
    /// - [`warnings`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::warnings)
    /// - [`source`](EuOwnGoodsTransfersComputeDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<EuOwnGoodsTransfersComputeDeclarationsResponse, BuildError> {
        Ok(EuOwnGoodsTransfersComputeDeclarationsResponse {
            period_year: self
                .period_year
                .ok_or_else(|| BuildError::missing_field("period_year"))?,
            period_month: self
                .period_month
                .ok_or_else(|| BuildError::missing_field("period_month"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            due_date: self
                .due_date
                .ok_or_else(|| BuildError::missing_field("due_date"))?,
            member_state_of_identification: self
                .member_state_of_identification
                .ok_or_else(|| BuildError::missing_field("member_state_of_identification"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
