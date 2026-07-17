pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsGetResponseLinesItemDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1PayrollRunsGetResponseLinesItemDeductionsItem {
    pub fn builder() -> PostV1PayrollRunsGetResponseLinesItemDeductionsItemBuilder {
        <PostV1PayrollRunsGetResponseLinesItemDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsGetResponseLinesItemDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl PostV1PayrollRunsGetResponseLinesItemDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsGetResponseLinesItemDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollRunsGetResponseLinesItemDeductionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollRunsGetResponseLinesItemDeductionsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1PayrollRunsGetResponseLinesItemDeductionsItem, BuildError> {
        Ok(PostV1PayrollRunsGetResponseLinesItemDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
