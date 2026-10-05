pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItem {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: NaiveDate,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    pub status: IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemStatus,
}

impl IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItem {
    pub fn builder(
    ) -> IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder {
        <IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder {
    invoice_id: Option<String>,
    document_number: Option<String>,
    document_date: Option<NaiveDate>,
    currency: Option<String>,
    gross_total: Option<String>,
    status:
        Option<IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemStatus>,
}

impl IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: NaiveDate) -> Self {
        self.document_date = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder::invoice_id)
    /// - [`document_number`](IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder::document_number)
    /// - [`document_date`](IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder::document_date)
    /// - [`currency`](IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder::currency)
    /// - [`gross_total`](IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder::gross_total)
    /// - [`status`](IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItemBuilder::status)
    pub fn build(
        self,
    ) -> Result<
        IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItem,
        BuildError,
    > {
        Ok(
            IntercompanyReportConsolidationResponseDirectionsItemUnmatchedPurchasesItem {
                invoice_id: self
                    .invoice_id
                    .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
                document_number: self
                    .document_number
                    .ok_or_else(|| BuildError::missing_field("document_number"))?,
                document_date: self
                    .document_date
                    .ok_or_else(|| BuildError::missing_field("document_date"))?,
                currency: self
                    .currency
                    .ok_or_else(|| BuildError::missing_field("currency"))?,
                gross_total: self
                    .gross_total
                    .ok_or_else(|| BuildError::missing_field("gross_total"))?,
                status: self
                    .status
                    .ok_or_else(|| BuildError::missing_field("status"))?,
            },
        )
    }
}
