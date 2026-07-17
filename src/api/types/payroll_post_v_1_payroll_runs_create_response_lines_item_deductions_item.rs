pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateResponseLinesItemDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1PayrollRunsCreateResponseLinesItemDeductionsItem {
    pub fn builder() -> PostV1PayrollRunsCreateResponseLinesItemDeductionsItemBuilder {
        <PostV1PayrollRunsCreateResponseLinesItemDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateResponseLinesItemDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl PostV1PayrollRunsCreateResponseLinesItemDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateResponseLinesItemDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollRunsCreateResponseLinesItemDeductionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollRunsCreateResponseLinesItemDeductionsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1PayrollRunsCreateResponseLinesItemDeductionsItem, BuildError> {
        Ok(PostV1PayrollRunsCreateResponseLinesItemDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
