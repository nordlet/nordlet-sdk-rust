pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuDigitalReportingListDeclarationsResponseTransactionsItem {
    pub direction: EuDigitalReportingListDeclarationsResponseTransactionsItemDirection,
    pub article: EuDigitalReportingListDeclarationsResponseTransactionsItemArticle,
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
    #[serde(rename = "documentType")]
    pub document_type: EuDigitalReportingListDeclarationsResponseTransactionsItemDocumentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "supplierVatNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_vat_number: Option<String>,
    #[serde(rename = "customerVatNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vat_number: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub lines: Vec<EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem>,
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<String>,
    #[serde(rename = "exemptionReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_reference: Option<String>,
    #[serde(rename = "reverseCharge")]
    #[serde(default)]
    pub reverse_charge: bool,
    #[serde(rename = "correctedInvoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corrected_invoice_number: Option<String>,
    #[serde(rename = "supplierAccounts")]
    #[serde(default)]
    pub supplier_accounts: Vec<String>,
    #[serde(rename = "reportTo")]
    #[serde(default)]
    pub report_to: String,
    #[serde(default)]
    pub deadline: String,
    #[serde(default)]
    pub missing: Vec<String>,
}

impl EuDigitalReportingListDeclarationsResponseTransactionsItem {
    pub fn builder() -> EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder {
        <EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder {
    direction: Option<EuDigitalReportingListDeclarationsResponseTransactionsItemDirection>,
    article: Option<EuDigitalReportingListDeclarationsResponseTransactionsItemArticle>,
    document_id: Option<String>,
    document_type: Option<EuDigitalReportingListDeclarationsResponseTransactionsItemDocumentType>,
    number: Option<String>,
    issue_date: Option<NaiveDate>,
    partner_name: Option<String>,
    supplier_vat_number: Option<String>,
    customer_vat_number: Option<String>,
    currency: Option<String>,
    lines: Option<Vec<EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem>>,
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
    exemption_reference: Option<String>,
    reverse_charge: Option<bool>,
    corrected_invoice_number: Option<String>,
    supplier_accounts: Option<Vec<String>>,
    report_to: Option<String>,
    deadline: Option<String>,
    missing: Option<Vec<String>>,
}

impl EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder {
    pub fn direction(
        mut self,
        value: EuDigitalReportingListDeclarationsResponseTransactionsItemDirection,
    ) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn article(
        mut self,
        value: EuDigitalReportingListDeclarationsResponseTransactionsItemArticle,
    ) -> Self {
        self.article = Some(value);
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn document_type(
        mut self,
        value: EuDigitalReportingListDeclarationsResponseTransactionsItemDocumentType,
    ) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn supplier_vat_number(mut self, value: impl Into<String>) -> Self {
        self.supplier_vat_number = Some(value.into());
        self
    }

    pub fn customer_vat_number(mut self, value: impl Into<String>) -> Self {
        self.customer_vat_number = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn lines(
        mut self,
        value: Vec<EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem>,
    ) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    pub fn exemption_reference(mut self, value: impl Into<String>) -> Self {
        self.exemption_reference = Some(value.into());
        self
    }

    pub fn reverse_charge(mut self, value: bool) -> Self {
        self.reverse_charge = Some(value);
        self
    }

    pub fn corrected_invoice_number(mut self, value: impl Into<String>) -> Self {
        self.corrected_invoice_number = Some(value.into());
        self
    }

    pub fn supplier_accounts(mut self, value: Vec<String>) -> Self {
        self.supplier_accounts = Some(value);
        self
    }

    pub fn report_to(mut self, value: impl Into<String>) -> Self {
        self.report_to = Some(value.into());
        self
    }

    pub fn deadline(mut self, value: impl Into<String>) -> Self {
        self.deadline = Some(value.into());
        self
    }

    pub fn missing(mut self, value: Vec<String>) -> Self {
        self.missing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuDigitalReportingListDeclarationsResponseTransactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`direction`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::direction)
    /// - [`article`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::article)
    /// - [`document_id`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::document_id)
    /// - [`document_type`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::document_type)
    /// - [`issue_date`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::issue_date)
    /// - [`partner_name`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::partner_name)
    /// - [`currency`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::currency)
    /// - [`lines`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::lines)
    /// - [`taxable_amount`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::taxable_amount)
    /// - [`reverse_charge`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::reverse_charge)
    /// - [`supplier_accounts`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::supplier_accounts)
    /// - [`report_to`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::report_to)
    /// - [`deadline`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::deadline)
    /// - [`missing`](EuDigitalReportingListDeclarationsResponseTransactionsItemBuilder::missing)
    pub fn build(
        self,
    ) -> Result<EuDigitalReportingListDeclarationsResponseTransactionsItem, BuildError> {
        Ok(EuDigitalReportingListDeclarationsResponseTransactionsItem {
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            article: self
                .article
                .ok_or_else(|| BuildError::missing_field("article"))?,
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            number: self.number,
            issue_date: self
                .issue_date
                .ok_or_else(|| BuildError::missing_field("issue_date"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            supplier_vat_number: self.supplier_vat_number,
            customer_vat_number: self.customer_vat_number,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            vat_amount: self.vat_amount,
            exemption_reference: self.exemption_reference,
            reverse_charge: self
                .reverse_charge
                .ok_or_else(|| BuildError::missing_field("reverse_charge"))?,
            corrected_invoice_number: self.corrected_invoice_number,
            supplier_accounts: self
                .supplier_accounts
                .ok_or_else(|| BuildError::missing_field("supplier_accounts"))?,
            report_to: self
                .report_to
                .ok_or_else(|| BuildError::missing_field("report_to"))?,
            deadline: self
                .deadline
                .ok_or_else(|| BuildError::missing_field("deadline"))?,
            missing: self
                .missing
                .ok_or_else(|| BuildError::missing_field("missing"))?,
        })
    }
}
