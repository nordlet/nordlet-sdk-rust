pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PeriodChecklistReportsResponseItemsItemCode {
    DraftSaleDocuments,
    DraftPurchaseDocuments,
    UnbookedBankLines,
    DepreciationNotPosted,
    DeferralsNotPosted,
    RecurringInvoicesNotGenerated,
    PayrollRunsNotApproved,
    OpenFxReceivables,
    OpenFxPayables,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PeriodChecklistReportsResponseItemsItemCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DraftSaleDocuments => serializer.serialize_str("draft_sale_documents"),
            Self::DraftPurchaseDocuments => serializer.serialize_str("draft_purchase_documents"),
            Self::UnbookedBankLines => serializer.serialize_str("unbooked_bank_lines"),
            Self::DepreciationNotPosted => serializer.serialize_str("depreciation_not_posted"),
            Self::DeferralsNotPosted => serializer.serialize_str("deferrals_not_posted"),
            Self::RecurringInvoicesNotGenerated => {
                serializer.serialize_str("recurring_invoices_not_generated")
            }
            Self::PayrollRunsNotApproved => serializer.serialize_str("payroll_runs_not_approved"),
            Self::OpenFxReceivables => serializer.serialize_str("open_fx_receivables"),
            Self::OpenFxPayables => serializer.serialize_str("open_fx_payables"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PeriodChecklistReportsResponseItemsItemCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft_sale_documents" => Ok(Self::DraftSaleDocuments),
            "draft_purchase_documents" => Ok(Self::DraftPurchaseDocuments),
            "unbooked_bank_lines" => Ok(Self::UnbookedBankLines),
            "depreciation_not_posted" => Ok(Self::DepreciationNotPosted),
            "deferrals_not_posted" => Ok(Self::DeferralsNotPosted),
            "recurring_invoices_not_generated" => Ok(Self::RecurringInvoicesNotGenerated),
            "payroll_runs_not_approved" => Ok(Self::PayrollRunsNotApproved),
            "open_fx_receivables" => Ok(Self::OpenFxReceivables),
            "open_fx_payables" => Ok(Self::OpenFxPayables),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PeriodChecklistReportsResponseItemsItemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DraftSaleDocuments => write!(f, "draft_sale_documents"),
            Self::DraftPurchaseDocuments => write!(f, "draft_purchase_documents"),
            Self::UnbookedBankLines => write!(f, "unbooked_bank_lines"),
            Self::DepreciationNotPosted => write!(f, "depreciation_not_posted"),
            Self::DeferralsNotPosted => write!(f, "deferrals_not_posted"),
            Self::RecurringInvoicesNotGenerated => write!(f, "recurring_invoices_not_generated"),
            Self::PayrollRunsNotApproved => write!(f, "payroll_runs_not_approved"),
            Self::OpenFxReceivables => write!(f, "open_fx_receivables"),
            Self::OpenFxPayables => write!(f, "open_fx_payables"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
