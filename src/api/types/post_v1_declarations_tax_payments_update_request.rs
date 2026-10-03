pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxPaymentsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<PostV1DeclarationsTaxPaymentsUpdateRequestKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "paidOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1DeclarationsTaxPaymentsUpdateRequest {
    pub fn builder() -> PostV1DeclarationsTaxPaymentsUpdateRequestBuilder {
        <PostV1DeclarationsTaxPaymentsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxPaymentsUpdateRequestBuilder {
    id: Option<String>,
    kind: Option<PostV1DeclarationsTaxPaymentsUpdateRequestKind>,
    amount: Option<String>,
    paid_on: Option<String>,
    reference: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsTaxPaymentsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1DeclarationsTaxPaymentsUpdateRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn paid_on(mut self, value: impl Into<String>) -> Self {
        self.paid_on = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxPaymentsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsTaxPaymentsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1DeclarationsTaxPaymentsUpdateRequest, BuildError> {
        Ok(PostV1DeclarationsTaxPaymentsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind,
            amount: self.amount,
            paid_on: self.paid_on,
            reference: self.reference,
            description: self.description,
        })
    }
}
