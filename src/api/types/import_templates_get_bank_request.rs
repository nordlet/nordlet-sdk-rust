pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesGetBankRequest {
    #[serde(default)]
    pub id: String,
}

impl ImportTemplatesGetBankRequest {
    pub fn builder() -> ImportTemplatesGetBankRequestBuilder {
        <ImportTemplatesGetBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesGetBankRequestBuilder {
    id: Option<String>,
}

impl ImportTemplatesGetBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesGetBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ImportTemplatesGetBankRequestBuilder::id)
    pub fn build(self) -> Result<ImportTemplatesGetBankRequest, BuildError> {
        Ok(ImportTemplatesGetBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
