pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccrualsCreateLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "reversalId")]
    #[serde(default)]
    pub reversal_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "reversalDate")]
    #[serde(default)]
    pub reversal_date: NaiveDate,
}

impl AccrualsCreateLedgerResponse {
    pub fn builder() -> AccrualsCreateLedgerResponseBuilder {
        <AccrualsCreateLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccrualsCreateLedgerResponseBuilder {
    id: Option<String>,
    reversal_id: Option<String>,
    date: Option<NaiveDate>,
    reversal_date: Option<NaiveDate>,
}

impl AccrualsCreateLedgerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reversal_id(mut self, value: impl Into<String>) -> Self {
        self.reversal_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn reversal_date(mut self, value: NaiveDate) -> Self {
        self.reversal_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccrualsCreateLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccrualsCreateLedgerResponseBuilder::id)
    /// - [`reversal_id`](AccrualsCreateLedgerResponseBuilder::reversal_id)
    /// - [`date`](AccrualsCreateLedgerResponseBuilder::date)
    /// - [`reversal_date`](AccrualsCreateLedgerResponseBuilder::reversal_date)
    pub fn build(self) -> Result<AccrualsCreateLedgerResponse, BuildError> {
        Ok(AccrualsCreateLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reversal_id: self
                .reversal_id
                .ok_or_else(|| BuildError::missing_field("reversal_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            reversal_date: self
                .reversal_date
                .ok_or_else(|| BuildError::missing_field("reversal_date"))?,
        })
    }
}
