pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlKsefReceivedListDeclarationsResponseRowsItem {
    #[serde(rename = "ksefReferenceNumber")]
    #[serde(default)]
    pub ksef_reference_number: String,
    #[serde(rename = "invoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    #[serde(rename = "issuerNip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_nip: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<NaiveDate>,
    #[serde(rename = "acquisitionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_timestamp: Option<String>,
    #[serde(rename = "grossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<String>,
    #[serde(rename = "purchaseInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_invoice_id: Option<String>,
}

impl PlKsefReceivedListDeclarationsResponseRowsItem {
    pub fn builder() -> PlKsefReceivedListDeclarationsResponseRowsItemBuilder {
        <PlKsefReceivedListDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlKsefReceivedListDeclarationsResponseRowsItemBuilder {
    ksef_reference_number: Option<String>,
    invoice_number: Option<String>,
    issuer_nip: Option<String>,
    issue_date: Option<NaiveDate>,
    acquisition_timestamp: Option<String>,
    gross_amount: Option<String>,
    purchase_invoice_id: Option<String>,
}

impl PlKsefReceivedListDeclarationsResponseRowsItemBuilder {
    pub fn ksef_reference_number(mut self, value: impl Into<String>) -> Self {
        self.ksef_reference_number = Some(value.into());
        self
    }

    pub fn invoice_number(mut self, value: impl Into<String>) -> Self {
        self.invoice_number = Some(value.into());
        self
    }

    pub fn issuer_nip(mut self, value: impl Into<String>) -> Self {
        self.issuer_nip = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn acquisition_timestamp(mut self, value: impl Into<String>) -> Self {
        self.acquisition_timestamp = Some(value.into());
        self
    }

    pub fn gross_amount(mut self, value: impl Into<String>) -> Self {
        self.gross_amount = Some(value.into());
        self
    }

    pub fn purchase_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_invoice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlKsefReceivedListDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ksef_reference_number`](PlKsefReceivedListDeclarationsResponseRowsItemBuilder::ksef_reference_number)
    pub fn build(self) -> Result<PlKsefReceivedListDeclarationsResponseRowsItem, BuildError> {
        Ok(PlKsefReceivedListDeclarationsResponseRowsItem {
            ksef_reference_number: self
                .ksef_reference_number
                .ok_or_else(|| BuildError::missing_field("ksef_reference_number"))?,
            invoice_number: self.invoice_number,
            issuer_nip: self.issuer_nip,
            issue_date: self.issue_date,
            acquisition_timestamp: self.acquisition_timestamp,
            gross_amount: self.gross_amount,
            purchase_invoice_id: self.purchase_invoice_id,
        })
    }
}
