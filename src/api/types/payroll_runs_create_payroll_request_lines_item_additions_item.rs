pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCreatePayrollRequestLinesItemAdditionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
}

impl RunsCreatePayrollRequestLinesItemAdditionsItem {
    pub fn builder() -> RunsCreatePayrollRequestLinesItemAdditionsItemBuilder {
        <RunsCreatePayrollRequestLinesItemAdditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCreatePayrollRequestLinesItemAdditionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
    taxable: Option<bool>,
}

impl RunsCreatePayrollRequestLinesItemAdditionsItemBuilder {
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

    /// Consumes the builder and constructs a [`RunsCreatePayrollRequestLinesItemAdditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RunsCreatePayrollRequestLinesItemAdditionsItemBuilder::name)
    /// - [`amount`](RunsCreatePayrollRequestLinesItemAdditionsItemBuilder::amount)
    pub fn build(self) -> Result<RunsCreatePayrollRequestLinesItemAdditionsItem, BuildError> {
        Ok(RunsCreatePayrollRequestLinesItemAdditionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            taxable: self.taxable,
        })
    }
}
