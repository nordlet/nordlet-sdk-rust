pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxPaymentsListRequest {
    pub tax: PostV1DeclarationsTaxPaymentsListRequestTax,
    #[serde(default)]
    pub year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
}

impl PostV1DeclarationsTaxPaymentsListRequest {
    pub fn builder() -> PostV1DeclarationsTaxPaymentsListRequestBuilder {
        <PostV1DeclarationsTaxPaymentsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxPaymentsListRequestBuilder {
    tax: Option<PostV1DeclarationsTaxPaymentsListRequestTax>,
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsTaxPaymentsListRequestBuilder {
    pub fn tax(mut self, value: PostV1DeclarationsTaxPaymentsListRequestTax) -> Self {
        self.tax = Some(value);
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxPaymentsListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tax`](PostV1DeclarationsTaxPaymentsListRequestBuilder::tax)
    /// - [`year`](PostV1DeclarationsTaxPaymentsListRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsTaxPaymentsListRequest, BuildError> {
        Ok(PostV1DeclarationsTaxPaymentsListRequest {
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self.month,
        })
    }
}
