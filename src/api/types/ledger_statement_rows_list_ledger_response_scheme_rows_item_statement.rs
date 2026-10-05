pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatementRowsListLedgerResponseSchemeRowsItemStatement {
    BalanceSheet,
    IncomeStatement,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StatementRowsListLedgerResponseSchemeRowsItemStatement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BalanceSheet => serializer.serialize_str("balance_sheet"),
            Self::IncomeStatement => serializer.serialize_str("income_statement"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StatementRowsListLedgerResponseSchemeRowsItemStatement {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "balance_sheet" => Ok(Self::BalanceSheet),
            "income_statement" => Ok(Self::IncomeStatement),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StatementRowsListLedgerResponseSchemeRowsItemStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BalanceSheet => write!(f, "balance_sheet"),
            Self::IncomeStatement => write!(f, "income_statement"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
