pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesReverseSalesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reason: String,
}

impl InvoicesReverseSalesRequest {
    pub fn builder() -> InvoicesReverseSalesRequestBuilder {
        <InvoicesReverseSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesReverseSalesRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl InvoicesReverseSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesReverseSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesReverseSalesRequestBuilder::id)
    /// - [`reason`](InvoicesReverseSalesRequestBuilder::reason)
    pub fn build(self) -> Result<InvoicesReverseSalesRequest, BuildError> {
        Ok(InvoicesReverseSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
