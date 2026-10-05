pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    pub status: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status:
        IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartPaymentStatus,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "amountsMatch")]
    #[serde(default)]
    pub amounts_match: bool,
}

impl IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart {
    pub fn builder(
    ) -> IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder {
        <IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder {
    invoice_id: Option<String>,
    status:
        Option<IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartStatus>,
    payment_status: Option<
        IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartPaymentStatus,
    >,
    gross_total: Option<String>,
    amounts_match: Option<bool>,
}

impl IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_status(
        mut self,
        value: IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartPaymentStatus,
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

    /// Consumes the builder and constructs a [`IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder::invoice_id)
    /// - [`status`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder::status)
    /// - [`payment_status`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder::payment_status)
    /// - [`gross_total`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder::gross_total)
    /// - [`amounts_match`](IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpartBuilder::amounts_match)
    pub fn build(
        self,
    ) -> Result<
        IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart,
        BuildError,
    > {
        Ok(
            IntercompanyReportConsolidationResponseDirectionsItemDocumentsItemCounterpart {
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
