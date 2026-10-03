pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlZusDraComputeResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub insured: String,
    #[serde(default)]
    pub payer: String,
    #[serde(default)]
    pub total: String,
}

impl PostV1DeclarationsPlZusDraComputeResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder {
        <PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    insured: Option<String>,
    payer: Option<String>,
    total: Option<String>,
}

impl PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn insured(mut self, value: impl Into<String>) -> Self {
        self.insured = Some(value.into());
        self
    }

    pub fn payer(mut self, value: impl Into<String>) -> Self {
        self.payer = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlZusDraComputeResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder::code)
    /// - [`label`](PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder::label)
    /// - [`insured`](PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder::insured)
    /// - [`payer`](PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder::payer)
    /// - [`total`](PostV1DeclarationsPlZusDraComputeResponseRowsItemBuilder::total)
    pub fn build(self) -> Result<PostV1DeclarationsPlZusDraComputeResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsPlZusDraComputeResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            insured: self
                .insured
                .ok_or_else(|| BuildError::missing_field("insured"))?,
            payer: self
                .payer
                .ok_or_else(|| BuildError::missing_field("payer"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
