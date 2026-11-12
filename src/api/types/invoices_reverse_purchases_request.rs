pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesReversePurchasesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl InvoicesReversePurchasesRequest {
    pub fn builder() -> InvoicesReversePurchasesRequestBuilder {
        <InvoicesReversePurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesReversePurchasesRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
    date: Option<NaiveDate>,
}

impl InvoicesReversePurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesReversePurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesReversePurchasesRequestBuilder::id)
    /// - [`reason`](InvoicesReversePurchasesRequestBuilder::reason)
    pub fn build(self) -> Result<InvoicesReversePurchasesRequest, BuildError> {
        Ok(InvoicesReversePurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            date: self.date,
        })
    }
}
