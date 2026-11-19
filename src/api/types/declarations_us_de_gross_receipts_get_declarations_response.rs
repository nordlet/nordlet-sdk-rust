pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsDeGrossReceiptsGetDeclarationsResponse {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub activity: String,
    #[serde(rename = "grossReceipts")]
    #[serde(default)]
    pub gross_receipts: String,
    #[serde(rename = "excludedReceipts")]
    #[serde(default)]
    pub excluded_receipts: String,
    #[serde(default)]
    pub deduction: String,
    #[serde(rename = "taxableReceipts")]
    #[serde(default)]
    pub taxable_receipts: String,
    #[serde(default)]
    pub rate: String,
    #[serde(default)]
    pub tax: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl UsDeGrossReceiptsGetDeclarationsResponse {
    pub fn builder() -> UsDeGrossReceiptsGetDeclarationsResponseBuilder {
        <UsDeGrossReceiptsGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeGrossReceiptsGetDeclarationsResponseBuilder {
    from: Option<String>,
    to: Option<String>,
    activity: Option<String>,
    gross_receipts: Option<String>,
    excluded_receipts: Option<String>,
    deduction: Option<String>,
    taxable_receipts: Option<String>,
    rate: Option<String>,
    tax: Option<String>,
    warnings: Option<Vec<String>>,
}

impl UsDeGrossReceiptsGetDeclarationsResponseBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn activity(mut self, value: impl Into<String>) -> Self {
        self.activity = Some(value.into());
        self
    }

    pub fn gross_receipts(mut self, value: impl Into<String>) -> Self {
        self.gross_receipts = Some(value.into());
        self
    }

    pub fn excluded_receipts(mut self, value: impl Into<String>) -> Self {
        self.excluded_receipts = Some(value.into());
        self
    }

    pub fn deduction(mut self, value: impl Into<String>) -> Self {
        self.deduction = Some(value.into());
        self
    }

    pub fn taxable_receipts(mut self, value: impl Into<String>) -> Self {
        self.taxable_receipts = Some(value.into());
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn tax(mut self, value: impl Into<String>) -> Self {
        self.tax = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsDeGrossReceiptsGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::from)
    /// - [`to`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::to)
    /// - [`activity`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::activity)
    /// - [`gross_receipts`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::gross_receipts)
    /// - [`excluded_receipts`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::excluded_receipts)
    /// - [`deduction`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::deduction)
    /// - [`taxable_receipts`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::taxable_receipts)
    /// - [`rate`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::rate)
    /// - [`tax`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::tax)
    /// - [`warnings`](UsDeGrossReceiptsGetDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<UsDeGrossReceiptsGetDeclarationsResponse, BuildError> {
        Ok(UsDeGrossReceiptsGetDeclarationsResponse {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            activity: self
                .activity
                .ok_or_else(|| BuildError::missing_field("activity"))?,
            gross_receipts: self
                .gross_receipts
                .ok_or_else(|| BuildError::missing_field("gross_receipts"))?,
            excluded_receipts: self
                .excluded_receipts
                .ok_or_else(|| BuildError::missing_field("excluded_receipts"))?,
            deduction: self
                .deduction
                .ok_or_else(|| BuildError::missing_field("deduction"))?,
            taxable_receipts: self
                .taxable_receipts
                .ok_or_else(|| BuildError::missing_field("taxable_receipts"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
