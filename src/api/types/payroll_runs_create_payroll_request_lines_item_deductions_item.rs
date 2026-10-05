pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCreatePayrollRequestLinesItemDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl RunsCreatePayrollRequestLinesItemDeductionsItem {
    pub fn builder() -> RunsCreatePayrollRequestLinesItemDeductionsItemBuilder {
        <RunsCreatePayrollRequestLinesItemDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCreatePayrollRequestLinesItemDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl RunsCreatePayrollRequestLinesItemDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsCreatePayrollRequestLinesItemDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RunsCreatePayrollRequestLinesItemDeductionsItemBuilder::name)
    /// - [`amount`](RunsCreatePayrollRequestLinesItemDeductionsItemBuilder::amount)
    pub fn build(self) -> Result<RunsCreatePayrollRequestLinesItemDeductionsItem, BuildError> {
        Ok(RunsCreatePayrollRequestLinesItemDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
