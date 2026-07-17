pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateResponseLinesItemAdditionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub taxable: bool,
}

impl PostV1PayrollRunsCreateResponseLinesItemAdditionsItem {
    pub fn builder() -> PostV1PayrollRunsCreateResponseLinesItemAdditionsItemBuilder {
        <PostV1PayrollRunsCreateResponseLinesItemAdditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateResponseLinesItemAdditionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
    taxable: Option<bool>,
}

impl PostV1PayrollRunsCreateResponseLinesItemAdditionsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateResponseLinesItemAdditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollRunsCreateResponseLinesItemAdditionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollRunsCreateResponseLinesItemAdditionsItemBuilder::amount)
    /// - [`taxable`](PostV1PayrollRunsCreateResponseLinesItemAdditionsItemBuilder::taxable)
    pub fn build(
        self,
    ) -> Result<PostV1PayrollRunsCreateResponseLinesItemAdditionsItem, BuildError> {
        Ok(PostV1PayrollRunsCreateResponseLinesItemAdditionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            taxable: self
                .taxable
                .ok_or_else(|| BuildError::missing_field("taxable"))?,
        })
    }
}
