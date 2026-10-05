pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IntercompanyReportConsolidationResponseDirectionsItemDocumentsItem {
    #[serde(rename = "sourceInvoiceId")]
    #[serde(default)]
    pub source_invoice_id: String,
    #[serde(rename = "fullNumber")]
    #[serde(default)]
    pub full_number: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    pub r#type: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemType,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "paymentStatus")]
    pub payment_status:
        IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemPaymentStatus,
    pub r#match: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemMatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterpart:
        Option<IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart>,
}

impl IntercompanyReportConsolidationResponseDirectionsItemDocumentsItem {
    pub fn builder() -> IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder {
        <IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder {
    source_invoice_id: Option<String>,
    full_number: Option<String>,
    issue_date: Option<NaiveDate>,
    r#type: Option<IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemType>,
    currency: Option<String>,
    gross_total: Option<String>,
    payment_status:
        Option<IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemPaymentStatus>,
    r#match: Option<IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemMatch>,
    counterpart:
        Option<IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart>,
}

impl IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder {
    pub fn source_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.source_invoice_id = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn r#type(
        mut self,
        value: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemType,
    ) -> Self {
        self.r#type = Some(value);
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

    pub fn payment_status(
        mut self,
        value: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemPaymentStatus,
    ) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn r#match(
        mut self,
        value: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemMatch,
    ) -> Self {
        self.r#match = Some(value);
        self
    }

    pub fn counterpart(
        mut self,
        value: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart,
    ) -> Self {
        self.counterpart = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyReportConsolidationResponseDirectionsItemDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_invoice_id`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::source_invoice_id)
    /// - [`full_number`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::full_number)
    /// - [`issue_date`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::issue_date)
    /// - [`r#type`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::r#type)
    /// - [`currency`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::currency)
    /// - [`gross_total`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::gross_total)
    /// - [`payment_status`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::payment_status)
    /// - [`r#match`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemBuilder::r#match)
    pub fn build(
        self,
    ) -> Result<IntercompanyReportConsolidationResponseDirectionsItemDocumentsItem, BuildError>
    {
        Ok(
            IntercompanyReportConsolidationResponseDirectionsItemDocumentsItem {
                source_invoice_id: self
                    .source_invoice_id
                    .ok_or_else(|| BuildError::missing_field("source_invoice_id"))?,
                full_number: self
                    .full_number
                    .ok_or_else(|| BuildError::missing_field("full_number"))?,
                issue_date: self
                    .issue_date
                    .ok_or_else(|| BuildError::missing_field("issue_date"))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| BuildError::missing_field("r#type"))?,
                currency: self
                    .currency
                    .ok_or_else(|| BuildError::missing_field("currency"))?,
                gross_total: self
                    .gross_total
                    .ok_or_else(|| BuildError::missing_field("gross_total"))?,
                payment_status: self
                    .payment_status
                    .ok_or_else(|| BuildError::missing_field("payment_status"))?,
                r#match: self
                    .r#match
                    .ok_or_else(|| BuildError::missing_field("r#match"))?,
                counterpart: self.counterpart,
            },
        )
    }
}
