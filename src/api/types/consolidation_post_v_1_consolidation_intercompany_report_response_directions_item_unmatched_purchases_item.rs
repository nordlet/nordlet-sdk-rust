pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    pub status:
        PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemStatus,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem {
    pub fn builder(
    ) -> PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder
    {
        <PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder
{
    invoice_id: Option<String>,
    document_number: Option<String>,
    document_date: Option<String>,
    currency: Option<String>,
    gross_total: Option<String>,
    status: Option<
        PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemStatus,
    >,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: impl Into<String>) -> Self {
        self.document_date = Some(value.into());
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
        value: PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder::invoice_id)
    /// - [`document_number`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder::document_number)
    /// - [`document_date`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder::document_date)
    /// - [`currency`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder::currency)
    /// - [`gross_total`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder::gross_total)
    /// - [`status`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItemBuilder::status)
    pub fn build(
        self,
    ) -> Result<
        PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem,
        BuildError,
    > {
        Ok(
            PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem {
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
