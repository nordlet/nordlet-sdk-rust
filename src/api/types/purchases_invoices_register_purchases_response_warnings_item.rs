pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesRegisterPurchasesResponseWarningsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub fix: String,
}

impl InvoicesRegisterPurchasesResponseWarningsItem {
    pub fn builder() -> InvoicesRegisterPurchasesResponseWarningsItemBuilder {
        <InvoicesRegisterPurchasesResponseWarningsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesRegisterPurchasesResponseWarningsItemBuilder {
    code: Option<String>,
    message: Option<String>,
    fix: Option<String>,
}

impl InvoicesRegisterPurchasesResponseWarningsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn fix(mut self, value: impl Into<String>) -> Self {
        self.fix = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesRegisterPurchasesResponseWarningsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](InvoicesRegisterPurchasesResponseWarningsItemBuilder::code)
    /// - [`message`](InvoicesRegisterPurchasesResponseWarningsItemBuilder::message)
    /// - [`fix`](InvoicesRegisterPurchasesResponseWarningsItemBuilder::fix)
    pub fn build(self) -> Result<InvoicesRegisterPurchasesResponseWarningsItem, BuildError> {
        Ok(InvoicesRegisterPurchasesResponseWarningsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            fix: self.fix.ok_or_else(|| BuildError::missing_field("fix"))?,
        })
    }
}
