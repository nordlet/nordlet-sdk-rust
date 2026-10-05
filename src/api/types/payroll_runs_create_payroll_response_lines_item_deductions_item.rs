pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCreatePayrollResponseLinesItemDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl RunsCreatePayrollResponseLinesItemDeductionsItem {
    pub fn builder() -> RunsCreatePayrollResponseLinesItemDeductionsItemBuilder {
        <RunsCreatePayrollResponseLinesItemDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCreatePayrollResponseLinesItemDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl RunsCreatePayrollResponseLinesItemDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsCreatePayrollResponseLinesItemDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RunsCreatePayrollResponseLinesItemDeductionsItemBuilder::name)
    /// - [`amount`](RunsCreatePayrollResponseLinesItemDeductionsItemBuilder::amount)
    pub fn build(self) -> Result<RunsCreatePayrollResponseLinesItemDeductionsItem, BuildError> {
        Ok(RunsCreatePayrollResponseLinesItemDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
