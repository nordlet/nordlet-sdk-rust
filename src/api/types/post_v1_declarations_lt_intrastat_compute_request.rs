pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIntrastatComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub flow: PostV1DeclarationsLtIntrastatComputeRequestFlow,
    #[serde(rename = "transactionNature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_nature: Option<String>,
    #[serde(rename = "deliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<String>,
    #[serde(rename = "transportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_mode: Option<PostV1DeclarationsLtIntrastatComputeRequestTransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
}

impl PostV1DeclarationsLtIntrastatComputeRequest {
    pub fn builder() -> PostV1DeclarationsLtIntrastatComputeRequestBuilder {
        <PostV1DeclarationsLtIntrastatComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIntrastatComputeRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    flow: Option<PostV1DeclarationsLtIntrastatComputeRequestFlow>,
    transaction_nature: Option<String>,
    delivery_terms: Option<String>,
    transport_mode: Option<PostV1DeclarationsLtIntrastatComputeRequestTransportMode>,
    persist: Option<bool>,
}

impl PostV1DeclarationsLtIntrastatComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn flow(mut self, value: PostV1DeclarationsLtIntrastatComputeRequestFlow) -> Self {
        self.flow = Some(value);
        self
    }

    pub fn transaction_nature(mut self, value: impl Into<String>) -> Self {
        self.transaction_nature = Some(value.into());
        self
    }

    pub fn delivery_terms(mut self, value: impl Into<String>) -> Self {
        self.delivery_terms = Some(value.into());
        self
    }

    pub fn transport_mode(
        mut self,
        value: PostV1DeclarationsLtIntrastatComputeRequestTransportMode,
    ) -> Self {
        self.transport_mode = Some(value);
        self
    }

    pub fn persist(mut self, value: bool) -> Self {
        self.persist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIntrastatComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtIntrastatComputeRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsLtIntrastatComputeRequestBuilder::month)
    /// - [`flow`](PostV1DeclarationsLtIntrastatComputeRequestBuilder::flow)
    pub fn build(self) -> Result<PostV1DeclarationsLtIntrastatComputeRequest, BuildError> {
        Ok(PostV1DeclarationsLtIntrastatComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            flow: self.flow.ok_or_else(|| BuildError::missing_field("flow"))?,
            transaction_nature: self.transaction_nature,
            delivery_terms: self.delivery_terms,
            transport_mode: self.transport_mode,
            persist: self.persist,
        })
    }
}
