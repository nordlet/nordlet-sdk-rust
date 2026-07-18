pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1LedgerPostingRulesUpdateRequestRulesItemKey {
    SalesReceivable,
    SalesRevenueProducts,
    SalesRevenueServices,
    SalesVatPayable,
    SalesAdvancesReceived,
    PurchasesPayables,
    PurchasesVatReceivable,
    PurchasesGoodsForResale,
    PurchasesDefaultExpense,
    InventoryCogs,
    InventoryStock,
    BankFxGain,
    BankFxLoss,
    SettlementsFees,
    SettlementsCommissionRevenue,
    SettlementsSellerPayable,
    SettlementsSuspense,
    RevenueDeferredIncome,
    RevenueContractAsset,
    RevenueRefundLiability,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1LedgerPostingRulesUpdateRequestRulesItemKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SalesReceivable => serializer.serialize_str("sales.receivable"),
            Self::SalesRevenueProducts => serializer.serialize_str("sales.revenueProducts"),
            Self::SalesRevenueServices => serializer.serialize_str("sales.revenueServices"),
            Self::SalesVatPayable => serializer.serialize_str("sales.vatPayable"),
            Self::SalesAdvancesReceived => serializer.serialize_str("sales.advancesReceived"),
            Self::PurchasesPayables => serializer.serialize_str("purchases.payables"),
            Self::PurchasesVatReceivable => serializer.serialize_str("purchases.vatReceivable"),
            Self::PurchasesGoodsForResale => serializer.serialize_str("purchases.goodsForResale"),
            Self::PurchasesDefaultExpense => serializer.serialize_str("purchases.defaultExpense"),
            Self::InventoryCogs => serializer.serialize_str("inventory.cogs"),
            Self::InventoryStock => serializer.serialize_str("inventory.stock"),
            Self::BankFxGain => serializer.serialize_str("bank.fxGain"),
            Self::BankFxLoss => serializer.serialize_str("bank.fxLoss"),
            Self::SettlementsFees => serializer.serialize_str("settlements.fees"),
            Self::SettlementsCommissionRevenue => {
                serializer.serialize_str("settlements.commissionRevenue")
            }
            Self::SettlementsSellerPayable => serializer.serialize_str("settlements.sellerPayable"),
            Self::SettlementsSuspense => serializer.serialize_str("settlements.suspense"),
            Self::RevenueDeferredIncome => serializer.serialize_str("revenue.deferredIncome"),
            Self::RevenueContractAsset => serializer.serialize_str("revenue.contractAsset"),
            Self::RevenueRefundLiability => serializer.serialize_str("revenue.refundLiability"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1LedgerPostingRulesUpdateRequestRulesItemKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sales.receivable" => Ok(Self::SalesReceivable),
            "sales.revenueProducts" => Ok(Self::SalesRevenueProducts),
            "sales.revenueServices" => Ok(Self::SalesRevenueServices),
            "sales.vatPayable" => Ok(Self::SalesVatPayable),
            "sales.advancesReceived" => Ok(Self::SalesAdvancesReceived),
            "purchases.payables" => Ok(Self::PurchasesPayables),
            "purchases.vatReceivable" => Ok(Self::PurchasesVatReceivable),
            "purchases.goodsForResale" => Ok(Self::PurchasesGoodsForResale),
            "purchases.defaultExpense" => Ok(Self::PurchasesDefaultExpense),
            "inventory.cogs" => Ok(Self::InventoryCogs),
            "inventory.stock" => Ok(Self::InventoryStock),
            "bank.fxGain" => Ok(Self::BankFxGain),
            "bank.fxLoss" => Ok(Self::BankFxLoss),
            "settlements.fees" => Ok(Self::SettlementsFees),
            "settlements.commissionRevenue" => Ok(Self::SettlementsCommissionRevenue),
            "settlements.sellerPayable" => Ok(Self::SettlementsSellerPayable),
            "settlements.suspense" => Ok(Self::SettlementsSuspense),
            "revenue.deferredIncome" => Ok(Self::RevenueDeferredIncome),
            "revenue.contractAsset" => Ok(Self::RevenueContractAsset),
            "revenue.refundLiability" => Ok(Self::RevenueRefundLiability),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1LedgerPostingRulesUpdateRequestRulesItemKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SalesReceivable => write!(f, "sales.receivable"),
            Self::SalesRevenueProducts => write!(f, "sales.revenueProducts"),
            Self::SalesRevenueServices => write!(f, "sales.revenueServices"),
            Self::SalesVatPayable => write!(f, "sales.vatPayable"),
            Self::SalesAdvancesReceived => write!(f, "sales.advancesReceived"),
            Self::PurchasesPayables => write!(f, "purchases.payables"),
            Self::PurchasesVatReceivable => write!(f, "purchases.vatReceivable"),
            Self::PurchasesGoodsForResale => write!(f, "purchases.goodsForResale"),
            Self::PurchasesDefaultExpense => write!(f, "purchases.defaultExpense"),
            Self::InventoryCogs => write!(f, "inventory.cogs"),
            Self::InventoryStock => write!(f, "inventory.stock"),
            Self::BankFxGain => write!(f, "bank.fxGain"),
            Self::BankFxLoss => write!(f, "bank.fxLoss"),
            Self::SettlementsFees => write!(f, "settlements.fees"),
            Self::SettlementsCommissionRevenue => write!(f, "settlements.commissionRevenue"),
            Self::SettlementsSellerPayable => write!(f, "settlements.sellerPayable"),
            Self::SettlementsSuspense => write!(f, "settlements.suspense"),
            Self::RevenueDeferredIncome => write!(f, "revenue.deferredIncome"),
            Self::RevenueContractAsset => write!(f, "revenue.contractAsset"),
            Self::RevenueRefundLiability => write!(f, "revenue.refundLiability"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
