pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMandatesUpdateRequest {
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

impl PostV1BankMandatesUpdateRequest {
    pub fn builder() -> PostV1BankMandatesUpdateRequestBuilder {
        <PostV1BankMandatesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMandatesUpdateRequestBuilder {
    bic: Option<String>,
    debtor_name: Option<String>,
    notes: Option<String>,
    id: Option<String>,
}

impl PostV1BankMandatesUpdateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1BankMandatesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMandatesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankMandatesUpdateRequest, BuildError> {
        Ok(PostV1BankMandatesUpdateRequest {
            bic: self.bic,
            debtor_name: self.debtor_name,
            notes: self.notes,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
