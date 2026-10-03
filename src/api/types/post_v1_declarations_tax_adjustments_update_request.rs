pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxAdjustmentsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<PostV1DeclarationsTaxAdjustmentsUpdateRequestKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1DeclarationsTaxAdjustmentsUpdateRequest {
    pub fn builder() -> PostV1DeclarationsTaxAdjustmentsUpdateRequestBuilder {
        <PostV1DeclarationsTaxAdjustmentsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxAdjustmentsUpdateRequestBuilder {
    id: Option<String>,
    kind: Option<PostV1DeclarationsTaxAdjustmentsUpdateRequestKind>,
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsTaxAdjustmentsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1DeclarationsTaxAdjustmentsUpdateRequestKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxAdjustmentsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsTaxAdjustmentsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1DeclarationsTaxAdjustmentsUpdateRequest, BuildError> {
        Ok(PostV1DeclarationsTaxAdjustmentsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind,
            code: self.code,
            amount: self.amount,
            description: self.description,
        })
    }
}
