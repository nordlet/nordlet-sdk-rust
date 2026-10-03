pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxAdjustmentsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    pub kind: PostV1DeclarationsTaxAdjustmentsUpdateResponseKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl PostV1DeclarationsTaxAdjustmentsUpdateResponse {
    pub fn builder() -> PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder {
        <PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder {
    id: Option<String>,
    year: Option<i64>,
    kind: Option<PostV1DeclarationsTaxAdjustmentsUpdateResponseKind>,
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn kind(mut self, value: PostV1DeclarationsTaxAdjustmentsUpdateResponseKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxAdjustmentsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder::id)
    /// - [`year`](PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder::year)
    /// - [`kind`](PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder::kind)
    /// - [`amount`](PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder::amount)
    /// - [`description`](PostV1DeclarationsTaxAdjustmentsUpdateResponseBuilder::description)
    pub fn build(self) -> Result<PostV1DeclarationsTaxAdjustmentsUpdateResponse, BuildError> {
        Ok(PostV1DeclarationsTaxAdjustmentsUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
