pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActsPdfSalesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ActsPdfSalesRequestLocale>,
}

impl ActsPdfSalesRequest {
    pub fn builder() -> ActsPdfSalesRequestBuilder {
        <ActsPdfSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsPdfSalesRequestBuilder {
    id: Option<String>,
    locale: Option<ActsPdfSalesRequestLocale>,
}

impl ActsPdfSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn locale(mut self, value: ActsPdfSalesRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActsPdfSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ActsPdfSalesRequestBuilder::id)
    pub fn build(self) -> Result<ActsPdfSalesRequest, BuildError> {
        Ok(ActsPdfSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            locale: self.locale,
        })
    }
}
