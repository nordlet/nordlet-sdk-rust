pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettlementsMatchBankRequest {
    #[serde(rename = "lineId")]
    #[serde(default)]
    pub line_id: String,
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
}

impl SettlementsMatchBankRequest {
    pub fn builder() -> SettlementsMatchBankRequestBuilder {
        <SettlementsMatchBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsMatchBankRequestBuilder {
    line_id: Option<String>,
    invoice_id: Option<String>,
}

impl SettlementsMatchBankRequestBuilder {
    pub fn line_id(mut self, value: impl Into<String>) -> Self {
        self.line_id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SettlementsMatchBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line_id`](SettlementsMatchBankRequestBuilder::line_id)
    pub fn build(self) -> Result<SettlementsMatchBankRequest, BuildError> {
        Ok(SettlementsMatchBankRequest {
            line_id: self
                .line_id
                .ok_or_else(|| BuildError::missing_field("line_id"))?,
            invoice_id: self.invoice_id,
        })
    }
}
