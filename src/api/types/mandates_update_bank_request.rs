pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MandatesUpdateBankRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(rename = "debtorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debtor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub id: String,
}

impl MandatesUpdateBankRequest {
    pub fn builder() -> MandatesUpdateBankRequestBuilder {
        <MandatesUpdateBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesUpdateBankRequestBuilder {
    bic: Option<String>,
    debtor_name: Option<String>,
    notes: Option<String>,
    id: Option<String>,
}

impl MandatesUpdateBankRequestBuilder {
    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn debtor_name(mut self, value: impl Into<String>) -> Self {
        self.debtor_name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MandatesUpdateBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MandatesUpdateBankRequestBuilder::id)
    pub fn build(self) -> Result<MandatesUpdateBankRequest, BuildError> {
        Ok(MandatesUpdateBankRequest {
            bic: self.bic,
            debtor_name: self.debtor_name,
            notes: self.notes,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
