pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtPln204ComputeResponseAnnexSItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl PostV1DeclarationsLtPln204ComputeResponseAnnexSItem {
    pub fn builder() -> PostV1DeclarationsLtPln204ComputeResponseAnnexSItemBuilder {
        <PostV1DeclarationsLtPln204ComputeResponseAnnexSItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtPln204ComputeResponseAnnexSItemBuilder {
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsLtPln204ComputeResponseAnnexSItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtPln204ComputeResponseAnnexSItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1DeclarationsLtPln204ComputeResponseAnnexSItemBuilder::code)
    /// - [`amount`](PostV1DeclarationsLtPln204ComputeResponseAnnexSItemBuilder::amount)
    /// - [`description`](PostV1DeclarationsLtPln204ComputeResponseAnnexSItemBuilder::description)
    pub fn build(self) -> Result<PostV1DeclarationsLtPln204ComputeResponseAnnexSItem, BuildError> {
        Ok(PostV1DeclarationsLtPln204ComputeResponseAnnexSItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
