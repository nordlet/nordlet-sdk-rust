pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentTypesListPayrollResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub applies: Vec<String>,
}

impl PaymentTypesListPayrollResponseRowsItem {
    pub fn builder() -> PaymentTypesListPayrollResponseRowsItemBuilder {
        <PaymentTypesListPayrollResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentTypesListPayrollResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    applies: Option<Vec<String>>,
}

impl PaymentTypesListPayrollResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn applies(mut self, value: Vec<String>) -> Self {
        self.applies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentTypesListPayrollResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PaymentTypesListPayrollResponseRowsItemBuilder::code)
    /// - [`name`](PaymentTypesListPayrollResponseRowsItemBuilder::name)
    /// - [`applies`](PaymentTypesListPayrollResponseRowsItemBuilder::applies)
    pub fn build(self) -> Result<PaymentTypesListPayrollResponseRowsItem, BuildError> {
        Ok(PaymentTypesListPayrollResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            applies: self
                .applies
                .ok_or_else(|| BuildError::missing_field("applies"))?,
        })
    }
}
