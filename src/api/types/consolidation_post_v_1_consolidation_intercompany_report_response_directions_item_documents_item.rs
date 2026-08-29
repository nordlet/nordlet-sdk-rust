pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem {
    #[serde(rename = "sourceInvoiceId")]
    #[serde(default)]
    pub source_invoice_id: String,
    #[serde(rename = "fullNumber")]
    #[serde(default)]
    pub full_number: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    pub r#type: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemType,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "paymentStatus")]
    pub payment_status:
        PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemPaymentStatus,
    pub r#match: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemMatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterpart:
        Option<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart>,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem {
    pub fn builder(
    ) -> PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder {
        <PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder {
    source_invoice_id: Option<String>,
    full_number: Option<String>,
    issue_date: Option<String>,
    r#type: Option<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemType>,
    currency: Option<String>,
    gross_total: Option<String>,
    payment_status: Option<
        PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemPaymentStatus,
    >,
    r#match: Option<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemMatch>,
    counterpart:
        Option<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart>,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder {
    pub fn source_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.source_invoice_id = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn r#type(
        mut self,
        value: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemType,
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
        value: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemPaymentStatus,
    ) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn r#match(
        mut self,
        value: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemMatch,
    ) -> Self {
        self.r#match = Some(value);
        self
    }

    pub fn counterpart(
        mut self,
        value: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart,
    ) -> Self {
        self.counterpart = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_invoice_id`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::source_invoice_id)
    /// - [`full_number`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::full_number)
    /// - [`issue_date`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::issue_date)
    /// - [`r#type`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::r#type)
    /// - [`currency`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::currency)
    /// - [`gross_total`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::gross_total)
    /// - [`payment_status`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::payment_status)
    /// - [`r#match`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemBuilder::r#match)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem, BuildError>
    {
        Ok(
            PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem {
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
