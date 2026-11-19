pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MandatesAuddisExportBankRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(rename = "mandateIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_ids: Option<Vec<String>>,
    #[serde(rename = "processingDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_date: Option<NaiveDate>,
}

impl MandatesAuddisExportBankRequest {
    pub fn builder() -> MandatesAuddisExportBankRequestBuilder {
        <MandatesAuddisExportBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesAuddisExportBankRequestBuilder {
    bank_account_id: Option<String>,
    mandate_ids: Option<Vec<String>>,
    processing_date: Option<NaiveDate>,
}

impl MandatesAuddisExportBankRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn mandate_ids(mut self, value: Vec<String>) -> Self {
        self.mandate_ids = Some(value);
        self
    }

    pub fn processing_date(mut self, value: NaiveDate) -> Self {
        self.processing_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MandatesAuddisExportBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](MandatesAuddisExportBankRequestBuilder::bank_account_id)
    pub fn build(self) -> Result<MandatesAuddisExportBankRequest, BuildError> {
        Ok(MandatesAuddisExportBankRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            mandate_ids: self.mandate_ids,
            processing_date: self.processing_date,
        })
    }
}
