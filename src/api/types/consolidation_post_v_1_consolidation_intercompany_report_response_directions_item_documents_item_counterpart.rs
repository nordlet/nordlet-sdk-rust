pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    pub status: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartPaymentStatus,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "amountsMatch")]
    #[serde(default)]
    pub amounts_match: bool,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart {
    pub fn builder(
    ) -> PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder
    {
        <PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder {
    invoice_id: Option<String>,
    status: Option<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartStatus>,
    payment_status: Option<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartPaymentStatus>,
    gross_total: Option<String>,
    amounts_match: Option<bool>,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_status(
        mut self,
        value: PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartPaymentStatus,
    ) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn amounts_match(mut self, value: bool) -> Self {
        self.amounts_match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder::invoice_id)
    /// - [`status`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder::status)
    /// - [`payment_status`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder::payment_status)
    /// - [`gross_total`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder::gross_total)
    /// - [`amounts_match`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpartBuilder::amounts_match)
    pub fn build(
        self,
    ) -> Result<
        PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart,
        BuildError,
    > {
        Ok(
            PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemCounterpart {
                invoice_id: self
                    .invoice_id
                    .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
                status: self
                    .status
                    .ok_or_else(|| BuildError::missing_field("status"))?,
                payment_status: self
                    .payment_status
                    .ok_or_else(|| BuildError::missing_field("payment_status"))?,
                gross_total: self
                    .gross_total
                    .ok_or_else(|| BuildError::missing_field("gross_total"))?,
                amounts_match: self
                    .amounts_match
                    .ok_or_else(|| BuildError::missing_field("amounts_match"))?,
            },
        )
    }
}
