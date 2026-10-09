pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpenseReportsCreateCashRequestLinesItem {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "documentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "netAmount")]
    #[serde(default)]
    pub net_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<String>,
}

impl ExpenseReportsCreateCashRequestLinesItem {
    pub fn builder() -> ExpenseReportsCreateCashRequestLinesItemBuilder {
        <ExpenseReportsCreateCashRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpenseReportsCreateCashRequestLinesItemBuilder {
    description: Option<String>,
    document_number: Option<String>,
    account_code: Option<String>,
    net_amount: Option<String>,
    vat_amount: Option<String>,
}

impl ExpenseReportsCreateCashRequestLinesItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn net_amount(mut self, value: impl Into<String>) -> Self {
        self.net_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExpenseReportsCreateCashRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](ExpenseReportsCreateCashRequestLinesItemBuilder::description)
    /// - [`account_code`](ExpenseReportsCreateCashRequestLinesItemBuilder::account_code)
    /// - [`net_amount`](ExpenseReportsCreateCashRequestLinesItemBuilder::net_amount)
    pub fn build(self) -> Result<ExpenseReportsCreateCashRequestLinesItem, BuildError> {
        Ok(ExpenseReportsCreateCashRequestLinesItem {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            document_number: self.document_number,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            vat_amount: self.vat_amount,
        })
    }
}
