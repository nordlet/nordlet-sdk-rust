pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateRequestLinesItemAdditionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
}

impl PostV1PayrollRunsCreateRequestLinesItemAdditionsItem {
    pub fn builder() -> PostV1PayrollRunsCreateRequestLinesItemAdditionsItemBuilder {
        <PostV1PayrollRunsCreateRequestLinesItemAdditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateRequestLinesItemAdditionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
    taxable: Option<bool>,
}

impl PostV1PayrollRunsCreateRequestLinesItemAdditionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn taxable(mut self, value: bool) -> Self {
        self.taxable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateRequestLinesItemAdditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollRunsCreateRequestLinesItemAdditionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollRunsCreateRequestLinesItemAdditionsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1PayrollRunsCreateRequestLinesItemAdditionsItem, BuildError> {
        Ok(PostV1PayrollRunsCreateRequestLinesItemAdditionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            taxable: self.taxable,
        })
    }
}
