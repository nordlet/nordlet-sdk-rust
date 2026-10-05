pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesIssueSalesResponseVatEvidenceRateTable {
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

impl InvoicesIssueSalesResponseVatEvidenceRateTable {
    pub fn builder() -> InvoicesIssueSalesResponseVatEvidenceRateTableBuilder {
        <InvoicesIssueSalesResponseVatEvidenceRateTableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesIssueSalesResponseVatEvidenceRateTableBuilder {
    import_id: Option<String>,
    situation_on: Option<String>,
    trigger: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
}

impl InvoicesIssueSalesResponseVatEvidenceRateTableBuilder {
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

    /// Consumes the builder and constructs a [`InvoicesIssueSalesResponseVatEvidenceRateTable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`import_id`](InvoicesIssueSalesResponseVatEvidenceRateTableBuilder::import_id)
    /// - [`situation_on`](InvoicesIssueSalesResponseVatEvidenceRateTableBuilder::situation_on)
    /// - [`trigger`](InvoicesIssueSalesResponseVatEvidenceRateTableBuilder::trigger)
    /// - [`started_at`](InvoicesIssueSalesResponseVatEvidenceRateTableBuilder::started_at)
    pub fn build(self) -> Result<InvoicesIssueSalesResponseVatEvidenceRateTable, BuildError> {
        Ok(InvoicesIssueSalesResponseVatEvidenceRateTable {
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
