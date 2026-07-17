pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateRequestLinesItemDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1PayrollRunsCreateRequestLinesItemDeductionsItem {
    pub fn builder() -> PostV1PayrollRunsCreateRequestLinesItemDeductionsItemBuilder {
        <PostV1PayrollRunsCreateRequestLinesItemDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateRequestLinesItemDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl PostV1PayrollRunsCreateRequestLinesItemDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateRequestLinesItemDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollRunsCreateRequestLinesItemDeductionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollRunsCreateRequestLinesItemDeductionsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1PayrollRunsCreateRequestLinesItemDeductionsItem, BuildError> {
        Ok(PostV1PayrollRunsCreateRequestLinesItemDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
