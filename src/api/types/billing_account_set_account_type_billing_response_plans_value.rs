pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountSetAccountTypeBillingResponsePlansValue {
    #[serde(rename = "monthlyFeeEur")]
    #[serde(default)]
    pub monthly_fee_eur: String,
    #[serde(rename = "includedInvoices")]
    #[serde(default)]
    pub included_invoices: i64,
    #[serde(rename = "invoiceOverageEur")]
    #[serde(default)]
    pub invoice_overage_eur: String,
    #[serde(rename = "includedDatabaseBytes")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub included_database_bytes: f64,
    #[serde(rename = "includedFileBytes")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub included_file_bytes: f64,
}

impl AccountSetAccountTypeBillingResponsePlansValue {
    pub fn builder() -> AccountSetAccountTypeBillingResponsePlansValueBuilder {
        <AccountSetAccountTypeBillingResponsePlansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetAccountTypeBillingResponsePlansValueBuilder {
    monthly_fee_eur: Option<String>,
    included_invoices: Option<i64>,
    invoice_overage_eur: Option<String>,
    included_database_bytes: Option<f64>,
    included_file_bytes: Option<f64>,
}

impl AccountSetAccountTypeBillingResponsePlansValueBuilder {
    pub fn monthly_fee_eur(mut self, value: impl Into<String>) -> Self {
        self.monthly_fee_eur = Some(value.into());
        self
    }

    pub fn included_invoices(mut self, value: i64) -> Self {
        self.included_invoices = Some(value);
        self
    }

    pub fn invoice_overage_eur(mut self, value: impl Into<String>) -> Self {
        self.invoice_overage_eur = Some(value.into());
        self
    }

    pub fn included_database_bytes(mut self, value: f64) -> Self {
        self.included_database_bytes = Some(value);
        self
    }

    pub fn included_file_bytes(mut self, value: f64) -> Self {
        self.included_file_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountSetAccountTypeBillingResponsePlansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`monthly_fee_eur`](AccountSetAccountTypeBillingResponsePlansValueBuilder::monthly_fee_eur)
    /// - [`included_invoices`](AccountSetAccountTypeBillingResponsePlansValueBuilder::included_invoices)
    /// - [`invoice_overage_eur`](AccountSetAccountTypeBillingResponsePlansValueBuilder::invoice_overage_eur)
    /// - [`included_database_bytes`](AccountSetAccountTypeBillingResponsePlansValueBuilder::included_database_bytes)
    /// - [`included_file_bytes`](AccountSetAccountTypeBillingResponsePlansValueBuilder::included_file_bytes)
    pub fn build(self) -> Result<AccountSetAccountTypeBillingResponsePlansValue, BuildError> {
        Ok(AccountSetAccountTypeBillingResponsePlansValue {
            monthly_fee_eur: self
                .monthly_fee_eur
                .ok_or_else(|| BuildError::missing_field("monthly_fee_eur"))?,
            included_invoices: self
                .included_invoices
                .ok_or_else(|| BuildError::missing_field("included_invoices"))?,
            invoice_overage_eur: self
                .invoice_overage_eur
                .ok_or_else(|| BuildError::missing_field("invoice_overage_eur"))?,
            included_database_bytes: self
                .included_database_bytes
                .ok_or_else(|| BuildError::missing_field("included_database_bytes"))?,
            included_file_bytes: self
                .included_file_bytes
                .ok_or_else(|| BuildError::missing_field("included_file_bytes"))?,
        })
    }
}
