pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtPln204ComputeResponseAnnexZItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub description: String,
}

impl PostV1DeclarationsLtPln204ComputeResponseAnnexZItem {
    pub fn builder() -> PostV1DeclarationsLtPln204ComputeResponseAnnexZItemBuilder {
        <PostV1DeclarationsLtPln204ComputeResponseAnnexZItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtPln204ComputeResponseAnnexZItemBuilder {
    code: Option<String>,
    amount: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsLtPln204ComputeResponseAnnexZItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtPln204ComputeResponseAnnexZItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1DeclarationsLtPln204ComputeResponseAnnexZItemBuilder::code)
    /// - [`amount`](PostV1DeclarationsLtPln204ComputeResponseAnnexZItemBuilder::amount)
    /// - [`description`](PostV1DeclarationsLtPln204ComputeResponseAnnexZItemBuilder::description)
    pub fn build(self) -> Result<PostV1DeclarationsLtPln204ComputeResponseAnnexZItem, BuildError> {
        Ok(PostV1DeclarationsLtPln204ComputeResponseAnnexZItem {
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
