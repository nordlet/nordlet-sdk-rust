pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesLockSalesResponseVatEvidenceRateTable {
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
    #[serde(rename = "situationOn")]
    #[serde(default)]
    pub situation_on: String,
    #[serde(default)]
    pub trigger: String,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
}

impl InvoicesLockSalesResponseVatEvidenceRateTable {
    pub fn builder() -> InvoicesLockSalesResponseVatEvidenceRateTableBuilder {
        <InvoicesLockSalesResponseVatEvidenceRateTableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesLockSalesResponseVatEvidenceRateTableBuilder {
    import_id: Option<String>,
    situation_on: Option<String>,
    trigger: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
}

impl InvoicesLockSalesResponseVatEvidenceRateTableBuilder {
    pub fn import_id(mut self, value: impl Into<String>) -> Self {
        self.import_id = Some(value.into());
        self
    }

    pub fn situation_on(mut self, value: impl Into<String>) -> Self {
        self.situation_on = Some(value.into());
        self
    }

    pub fn trigger(mut self, value: impl Into<String>) -> Self {
        self.trigger = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesLockSalesResponseVatEvidenceRateTable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`import_id`](InvoicesLockSalesResponseVatEvidenceRateTableBuilder::import_id)
    /// - [`situation_on`](InvoicesLockSalesResponseVatEvidenceRateTableBuilder::situation_on)
    /// - [`trigger`](InvoicesLockSalesResponseVatEvidenceRateTableBuilder::trigger)
    /// - [`started_at`](InvoicesLockSalesResponseVatEvidenceRateTableBuilder::started_at)
    pub fn build(self) -> Result<InvoicesLockSalesResponseVatEvidenceRateTable, BuildError> {
        Ok(InvoicesLockSalesResponseVatEvidenceRateTable {
            import_id: self
                .import_id
                .ok_or_else(|| BuildError::missing_field("import_id"))?,
            situation_on: self
                .situation_on
                .ok_or_else(|| BuildError::missing_field("situation_on"))?,
            trigger: self
                .trigger
                .ok_or_else(|| BuildError::missing_field("trigger"))?,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
        })
    }
}
