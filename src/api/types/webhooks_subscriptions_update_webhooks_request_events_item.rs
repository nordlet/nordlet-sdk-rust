pub use crate::prelude::*;

/// An event type, or "*" for every event
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionsUpdateWebhooksRequestEventsItem {
    AgreementInvoiceGenerated,
    BankFeedSynced,
    DocumentCapturePeppolReceived,
    FilingFailed,
    FilingRejected,
    GoodsReceiptPosted,
    IntercompanyInvoiceMirrored,
    ItemCreated,
    ItemDeleted,
    ItemUpdated,
    LeadConverted,
    LeadCreated,
    PartnerInquiryCreated,
    PayrollRunApproved,
    PayrollRunReversed,
    PosReportCreated,
    PriceListUpdated,
    PurchaseInvoicePaid,
    PurchaseInvoiceRegistered,
    PurchaseOrderApproved,
    PurchaseOrderReceived,
    RefundLiabilityActual,
    RefundLiabilityTruedUp,
    ReportCompleted,
    ReportFailed,
    RevenueRecognitionModified,
    RevenueRecognitionPosted,
    SaleInvoiceEinvoiceSent,
    SaleInvoiceIssued,
    SaleInvoicePaid,
    SaleInvoicePeppolDelivered,
    SaleInvoicePeppolFailed,
    SaleInvoicePeppolRejected,
    SaleInvoicePeppolSent,
    SaleInvoiceSent,
    SalesOrderCreated,
    SalesOrderFulfilled,
    SettlementImported,
    SettlementPosted,
    SettlementUpdated,
    StockChanged,
    StockReorderNeeded,
    VatReviewOpened,
    VatReviewResolved,
    All,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionsUpdateWebhooksRequestEventsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AgreementInvoiceGenerated => {
                serializer.serialize_str("agreement.invoice_generated")
            }
            Self::BankFeedSynced => serializer.serialize_str("bank_feed.synced"),
            Self::DocumentCapturePeppolReceived => {
                serializer.serialize_str("document_capture.peppol_received")
            }
            Self::FilingFailed => serializer.serialize_str("filing.failed"),
            Self::FilingRejected => serializer.serialize_str("filing.rejected"),
            Self::GoodsReceiptPosted => serializer.serialize_str("goods_receipt.posted"),
            Self::IntercompanyInvoiceMirrored => {
                serializer.serialize_str("intercompany.invoice_mirrored")
            }
            Self::ItemCreated => serializer.serialize_str("item.created"),
            Self::ItemDeleted => serializer.serialize_str("item.deleted"),
            Self::ItemUpdated => serializer.serialize_str("item.updated"),
            Self::LeadConverted => serializer.serialize_str("lead.converted"),
            Self::LeadCreated => serializer.serialize_str("lead.created"),
            Self::PartnerInquiryCreated => serializer.serialize_str("partner_inquiry.created"),
            Self::PayrollRunApproved => serializer.serialize_str("payroll_run.approved"),
            Self::PayrollRunReversed => serializer.serialize_str("payroll_run.reversed"),
            Self::PosReportCreated => serializer.serialize_str("pos_report.created"),
            Self::PriceListUpdated => serializer.serialize_str("price_list.updated"),
            Self::PurchaseInvoicePaid => serializer.serialize_str("purchase_invoice.paid"),
            Self::PurchaseInvoiceRegistered => {
                serializer.serialize_str("purchase_invoice.registered")
            }
            Self::PurchaseOrderApproved => serializer.serialize_str("purchase_order.approved"),
            Self::PurchaseOrderReceived => serializer.serialize_str("purchase_order.received"),
            Self::RefundLiabilityActual => serializer.serialize_str("refund_liability.actual"),
            Self::RefundLiabilityTruedUp => serializer.serialize_str("refund_liability.trued_up"),
            Self::ReportCompleted => serializer.serialize_str("report.completed"),
            Self::ReportFailed => serializer.serialize_str("report.failed"),
            Self::RevenueRecognitionModified => {
                serializer.serialize_str("revenue_recognition.modified")
            }
            Self::RevenueRecognitionPosted => {
                serializer.serialize_str("revenue_recognition.posted")
            }
            Self::SaleInvoiceEinvoiceSent => serializer.serialize_str("sale_invoice.einvoice_sent"),
            Self::SaleInvoiceIssued => serializer.serialize_str("sale_invoice.issued"),
            Self::SaleInvoicePaid => serializer.serialize_str("sale_invoice.paid"),
            Self::SaleInvoicePeppolDelivered => {
                serializer.serialize_str("sale_invoice.peppol_delivered")
            }
            Self::SaleInvoicePeppolFailed => serializer.serialize_str("sale_invoice.peppol_failed"),
            Self::SaleInvoicePeppolRejected => {
                serializer.serialize_str("sale_invoice.peppol_rejected")
            }
            Self::SaleInvoicePeppolSent => serializer.serialize_str("sale_invoice.peppol_sent"),
            Self::SaleInvoiceSent => serializer.serialize_str("sale_invoice.sent"),
            Self::SalesOrderCreated => serializer.serialize_str("sales_order.created"),
            Self::SalesOrderFulfilled => serializer.serialize_str("sales_order.fulfilled"),
            Self::SettlementImported => serializer.serialize_str("settlement.imported"),
            Self::SettlementPosted => serializer.serialize_str("settlement.posted"),
            Self::SettlementUpdated => serializer.serialize_str("settlement.updated"),
            Self::StockChanged => serializer.serialize_str("stock.changed"),
            Self::StockReorderNeeded => serializer.serialize_str("stock.reorder_needed"),
            Self::VatReviewOpened => serializer.serialize_str("vat_review.opened"),
            Self::VatReviewResolved => serializer.serialize_str("vat_review.resolved"),
            Self::All => serializer.serialize_str("*"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionsUpdateWebhooksRequestEventsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "agreement.invoice_generated" => Ok(Self::AgreementInvoiceGenerated),
            "bank_feed.synced" => Ok(Self::BankFeedSynced),
            "document_capture.peppol_received" => Ok(Self::DocumentCapturePeppolReceived),
            "filing.failed" => Ok(Self::FilingFailed),
            "filing.rejected" => Ok(Self::FilingRejected),
            "goods_receipt.posted" => Ok(Self::GoodsReceiptPosted),
            "intercompany.invoice_mirrored" => Ok(Self::IntercompanyInvoiceMirrored),
            "item.created" => Ok(Self::ItemCreated),
            "item.deleted" => Ok(Self::ItemDeleted),
            "item.updated" => Ok(Self::ItemUpdated),
            "lead.converted" => Ok(Self::LeadConverted),
            "lead.created" => Ok(Self::LeadCreated),
            "partner_inquiry.created" => Ok(Self::PartnerInquiryCreated),
            "payroll_run.approved" => Ok(Self::PayrollRunApproved),
            "payroll_run.reversed" => Ok(Self::PayrollRunReversed),
            "pos_report.created" => Ok(Self::PosReportCreated),
            "price_list.updated" => Ok(Self::PriceListUpdated),
            "purchase_invoice.paid" => Ok(Self::PurchaseInvoicePaid),
            "purchase_invoice.registered" => Ok(Self::PurchaseInvoiceRegistered),
            "purchase_order.approved" => Ok(Self::PurchaseOrderApproved),
            "purchase_order.received" => Ok(Self::PurchaseOrderReceived),
            "refund_liability.actual" => Ok(Self::RefundLiabilityActual),
            "refund_liability.trued_up" => Ok(Self::RefundLiabilityTruedUp),
            "report.completed" => Ok(Self::ReportCompleted),
            "report.failed" => Ok(Self::ReportFailed),
            "revenue_recognition.modified" => Ok(Self::RevenueRecognitionModified),
            "revenue_recognition.posted" => Ok(Self::RevenueRecognitionPosted),
            "sale_invoice.einvoice_sent" => Ok(Self::SaleInvoiceEinvoiceSent),
            "sale_invoice.issued" => Ok(Self::SaleInvoiceIssued),
            "sale_invoice.paid" => Ok(Self::SaleInvoicePaid),
            "sale_invoice.peppol_delivered" => Ok(Self::SaleInvoicePeppolDelivered),
            "sale_invoice.peppol_failed" => Ok(Self::SaleInvoicePeppolFailed),
            "sale_invoice.peppol_rejected" => Ok(Self::SaleInvoicePeppolRejected),
            "sale_invoice.peppol_sent" => Ok(Self::SaleInvoicePeppolSent),
            "sale_invoice.sent" => Ok(Self::SaleInvoiceSent),
            "sales_order.created" => Ok(Self::SalesOrderCreated),
            "sales_order.fulfilled" => Ok(Self::SalesOrderFulfilled),
            "settlement.imported" => Ok(Self::SettlementImported),
            "settlement.posted" => Ok(Self::SettlementPosted),
            "settlement.updated" => Ok(Self::SettlementUpdated),
            "stock.changed" => Ok(Self::StockChanged),
            "stock.reorder_needed" => Ok(Self::StockReorderNeeded),
            "vat_review.opened" => Ok(Self::VatReviewOpened),
            "vat_review.resolved" => Ok(Self::VatReviewResolved),
            "*" => Ok(Self::All),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionsUpdateWebhooksRequestEventsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgreementInvoiceGenerated => write!(f, "agreement.invoice_generated"),
            Self::BankFeedSynced => write!(f, "bank_feed.synced"),
            Self::DocumentCapturePeppolReceived => write!(f, "document_capture.peppol_received"),
            Self::FilingFailed => write!(f, "filing.failed"),
            Self::FilingRejected => write!(f, "filing.rejected"),
            Self::GoodsReceiptPosted => write!(f, "goods_receipt.posted"),
            Self::IntercompanyInvoiceMirrored => write!(f, "intercompany.invoice_mirrored"),
            Self::ItemCreated => write!(f, "item.created"),
            Self::ItemDeleted => write!(f, "item.deleted"),
            Self::ItemUpdated => write!(f, "item.updated"),
            Self::LeadConverted => write!(f, "lead.converted"),
            Self::LeadCreated => write!(f, "lead.created"),
            Self::PartnerInquiryCreated => write!(f, "partner_inquiry.created"),
            Self::PayrollRunApproved => write!(f, "payroll_run.approved"),
            Self::PayrollRunReversed => write!(f, "payroll_run.reversed"),
            Self::PosReportCreated => write!(f, "pos_report.created"),
            Self::PriceListUpdated => write!(f, "price_list.updated"),
            Self::PurchaseInvoicePaid => write!(f, "purchase_invoice.paid"),
            Self::PurchaseInvoiceRegistered => write!(f, "purchase_invoice.registered"),
            Self::PurchaseOrderApproved => write!(f, "purchase_order.approved"),
            Self::PurchaseOrderReceived => write!(f, "purchase_order.received"),
            Self::RefundLiabilityActual => write!(f, "refund_liability.actual"),
            Self::RefundLiabilityTruedUp => write!(f, "refund_liability.trued_up"),
            Self::ReportCompleted => write!(f, "report.completed"),
            Self::ReportFailed => write!(f, "report.failed"),
            Self::RevenueRecognitionModified => write!(f, "revenue_recognition.modified"),
            Self::RevenueRecognitionPosted => write!(f, "revenue_recognition.posted"),
            Self::SaleInvoiceEinvoiceSent => write!(f, "sale_invoice.einvoice_sent"),
            Self::SaleInvoiceIssued => write!(f, "sale_invoice.issued"),
            Self::SaleInvoicePaid => write!(f, "sale_invoice.paid"),
            Self::SaleInvoicePeppolDelivered => write!(f, "sale_invoice.peppol_delivered"),
            Self::SaleInvoicePeppolFailed => write!(f, "sale_invoice.peppol_failed"),
            Self::SaleInvoicePeppolRejected => write!(f, "sale_invoice.peppol_rejected"),
            Self::SaleInvoicePeppolSent => write!(f, "sale_invoice.peppol_sent"),
            Self::SaleInvoiceSent => write!(f, "sale_invoice.sent"),
            Self::SalesOrderCreated => write!(f, "sales_order.created"),
            Self::SalesOrderFulfilled => write!(f, "sales_order.fulfilled"),
            Self::SettlementImported => write!(f, "settlement.imported"),
            Self::SettlementPosted => write!(f, "settlement.posted"),
            Self::SettlementUpdated => write!(f, "settlement.updated"),
            Self::StockChanged => write!(f, "stock.changed"),
            Self::StockReorderNeeded => write!(f, "stock.reorder_needed"),
            Self::VatReviewOpened => write!(f, "vat_review.opened"),
            Self::VatReviewResolved => write!(f, "vat_review.resolved"),
            Self::All => write!(f, "*"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
