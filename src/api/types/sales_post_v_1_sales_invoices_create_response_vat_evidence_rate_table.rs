pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesCreateResponseVatEvidenceRateTable {
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
    pub started_at: String,
}

impl PostV1SalesInvoicesCreateResponseVatEvidenceRateTable {
    pub fn builder() -> PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder {
        <PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder {
    import_id: Option<String>,
    situation_on: Option<String>,
    trigger: Option<String>,
    started_at: Option<String>,
}

impl PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder {
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

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesCreateResponseVatEvidenceRateTable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`import_id`](PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder::import_id)
    /// - [`situation_on`](PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder::situation_on)
    /// - [`trigger`](PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder::trigger)
    /// - [`started_at`](PostV1SalesInvoicesCreateResponseVatEvidenceRateTableBuilder::started_at)
    pub fn build(
        self,
    ) -> Result<PostV1SalesInvoicesCreateResponseVatEvidenceRateTable, BuildError> {
        Ok(PostV1SalesInvoicesCreateResponseVatEvidenceRateTable {
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
