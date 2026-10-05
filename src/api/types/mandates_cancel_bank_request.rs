pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MandatesCancelBankRequest {
    #[serde(default)]
    pub id: String,
}

impl MandatesCancelBankRequest {
    pub fn builder() -> MandatesCancelBankRequestBuilder {
        <MandatesCancelBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesCancelBankRequestBuilder {
    id: Option<String>,
}

impl MandatesCancelBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MandatesCancelBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MandatesCancelBankRequestBuilder::id)
    pub fn build(self) -> Result<MandatesCancelBankRequest, BuildError> {
        Ok(MandatesCancelBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
