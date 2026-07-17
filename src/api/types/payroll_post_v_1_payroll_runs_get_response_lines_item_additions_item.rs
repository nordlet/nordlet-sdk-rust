pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsGetResponseLinesItemAdditionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub taxable: bool,
}

impl PostV1PayrollRunsGetResponseLinesItemAdditionsItem {
    pub fn builder() -> PostV1PayrollRunsGetResponseLinesItemAdditionsItemBuilder {
        <PostV1PayrollRunsGetResponseLinesItemAdditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsGetResponseLinesItemAdditionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
    taxable: Option<bool>,
}

impl PostV1PayrollRunsGetResponseLinesItemAdditionsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PayrollRunsGetResponseLinesItemAdditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollRunsGetResponseLinesItemAdditionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollRunsGetResponseLinesItemAdditionsItemBuilder::amount)
    /// - [`taxable`](PostV1PayrollRunsGetResponseLinesItemAdditionsItemBuilder::taxable)
    pub fn build(self) -> Result<PostV1PayrollRunsGetResponseLinesItemAdditionsItem, BuildError> {
        Ok(PostV1PayrollRunsGetResponseLinesItemAdditionsItem {
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
