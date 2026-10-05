pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsUpdateDeclarationsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TaxAdjustmentsUpdateDeclarationsRequestKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TaxAdjustmentsUpdateDeclarationsRequest {
    pub fn builder() -> TaxAdjustmentsUpdateDeclarationsRequestBuilder {
        <TaxAdjustmentsUpdateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsUpdateDeclarationsRequestBuilder {
    id: Option<String>,
    kind: Option<TaxAdjustmentsUpdateDeclarationsRequestKind>,
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl TaxAdjustmentsUpdateDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: TaxAdjustmentsUpdateDeclarationsRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxAdjustmentsUpdateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxAdjustmentsUpdateDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<TaxAdjustmentsUpdateDeclarationsRequest, BuildError> {
        Ok(TaxAdjustmentsUpdateDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind,
            code: self.code,
            amount: self.amount,
            description: self.description,
        })
    }
}
