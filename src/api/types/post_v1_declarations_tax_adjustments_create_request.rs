pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxAdjustmentsCreateRequest {
    #[serde(default)]
    pub year: i64,
    pub kind: PostV1DeclarationsTaxAdjustmentsCreateRequestKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl PostV1DeclarationsTaxAdjustmentsCreateRequest {
    pub fn builder() -> PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder {
        <PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder {
    year: Option<i64>,
    kind: Option<PostV1DeclarationsTaxAdjustmentsCreateRequestKind>,
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn kind(mut self, value: PostV1DeclarationsTaxAdjustmentsCreateRequestKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxAdjustmentsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder::year)
    /// - [`kind`](PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder::kind)
    /// - [`amount`](PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder::amount)
    /// - [`description`](PostV1DeclarationsTaxAdjustmentsCreateRequestBuilder::description)
    pub fn build(self) -> Result<PostV1DeclarationsTaxAdjustmentsCreateRequest, BuildError> {
        Ok(PostV1DeclarationsTaxAdjustmentsCreateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            code: self.code,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
