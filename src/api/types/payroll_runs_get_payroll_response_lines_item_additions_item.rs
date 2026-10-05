pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsGetPayrollResponseLinesItemAdditionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub taxable: bool,
}

impl RunsGetPayrollResponseLinesItemAdditionsItem {
    pub fn builder() -> RunsGetPayrollResponseLinesItemAdditionsItemBuilder {
        <RunsGetPayrollResponseLinesItemAdditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsGetPayrollResponseLinesItemAdditionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
    taxable: Option<bool>,
}

impl RunsGetPayrollResponseLinesItemAdditionsItemBuilder {
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

    /// Consumes the builder and constructs a [`RunsGetPayrollResponseLinesItemAdditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RunsGetPayrollResponseLinesItemAdditionsItemBuilder::name)
    /// - [`amount`](RunsGetPayrollResponseLinesItemAdditionsItemBuilder::amount)
    /// - [`taxable`](RunsGetPayrollResponseLinesItemAdditionsItemBuilder::taxable)
    pub fn build(self) -> Result<RunsGetPayrollResponseLinesItemAdditionsItem, BuildError> {
        Ok(RunsGetPayrollResponseLinesItemAdditionsItem {
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
