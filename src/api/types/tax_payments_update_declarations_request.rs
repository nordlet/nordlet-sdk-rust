pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxPaymentsUpdateDeclarationsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TaxPaymentsUpdateDeclarationsRequestKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "paidOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_on: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TaxPaymentsUpdateDeclarationsRequest {
    pub fn builder() -> TaxPaymentsUpdateDeclarationsRequestBuilder {
        <TaxPaymentsUpdateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxPaymentsUpdateDeclarationsRequestBuilder {
    id: Option<String>,
    kind: Option<TaxPaymentsUpdateDeclarationsRequestKind>,
    amount: Option<String>,
    paid_on: Option<NaiveDate>,
    reference: Option<String>,
    description: Option<String>,
}

impl TaxPaymentsUpdateDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: TaxPaymentsUpdateDeclarationsRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn paid_on(mut self, value: NaiveDate) -> Self {
        self.paid_on = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxPaymentsUpdateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxPaymentsUpdateDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<TaxPaymentsUpdateDeclarationsRequest, BuildError> {
        Ok(TaxPaymentsUpdateDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind,
            amount: self.amount,
            paid_on: self.paid_on,
            reference: self.reference,
            description: self.description,
        })
    }
}
