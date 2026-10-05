pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsGenerateInvoiceAgreementsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<NaiveDate>,
}

impl AgreementsGenerateInvoiceAgreementsRequest {
    pub fn builder() -> AgreementsGenerateInvoiceAgreementsRequestBuilder {
        <AgreementsGenerateInvoiceAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsGenerateInvoiceAgreementsRequestBuilder {
    id: Option<String>,
    as_of_date: Option<NaiveDate>,
}

impl AgreementsGenerateInvoiceAgreementsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn as_of_date(mut self, value: NaiveDate) -> Self {
        self.as_of_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgreementsGenerateInvoiceAgreementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgreementsGenerateInvoiceAgreementsRequestBuilder::id)
    pub fn build(self) -> Result<AgreementsGenerateInvoiceAgreementsRequest, BuildError> {
        Ok(AgreementsGenerateInvoiceAgreementsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            as_of_date: self.as_of_date,
        })
    }
}
