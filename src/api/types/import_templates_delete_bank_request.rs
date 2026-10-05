pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesDeleteBankRequest {
    #[serde(default)]
    pub id: String,
}

impl ImportTemplatesDeleteBankRequest {
    pub fn builder() -> ImportTemplatesDeleteBankRequestBuilder {
        <ImportTemplatesDeleteBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesDeleteBankRequestBuilder {
    id: Option<String>,
}

impl ImportTemplatesDeleteBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesDeleteBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ImportTemplatesDeleteBankRequestBuilder::id)
    pub fn build(self) -> Result<ImportTemplatesDeleteBankRequest, BuildError> {
        Ok(ImportTemplatesDeleteBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
