pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuVatReturnComputeResponseBoxesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1DeclarationsEuVatReturnComputeResponseBoxesItem {
    pub fn builder() -> PostV1DeclarationsEuVatReturnComputeResponseBoxesItemBuilder {
        <PostV1DeclarationsEuVatReturnComputeResponseBoxesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuVatReturnComputeResponseBoxesItemBuilder {
    code: Option<String>,
    label: Option<String>,
    amount: Option<String>,
}

impl PostV1DeclarationsEuVatReturnComputeResponseBoxesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuVatReturnComputeResponseBoxesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1DeclarationsEuVatReturnComputeResponseBoxesItemBuilder::code)
    /// - [`label`](PostV1DeclarationsEuVatReturnComputeResponseBoxesItemBuilder::label)
    /// - [`amount`](PostV1DeclarationsEuVatReturnComputeResponseBoxesItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuVatReturnComputeResponseBoxesItem, BuildError> {
        Ok(PostV1DeclarationsEuVatReturnComputeResponseBoxesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
