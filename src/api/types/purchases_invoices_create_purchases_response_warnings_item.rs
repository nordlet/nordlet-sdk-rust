pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCreatePurchasesResponseWarningsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl InvoicesCreatePurchasesResponseWarningsItem {
    pub fn builder() -> InvoicesCreatePurchasesResponseWarningsItemBuilder {
        <InvoicesCreatePurchasesResponseWarningsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCreatePurchasesResponseWarningsItemBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl InvoicesCreatePurchasesResponseWarningsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCreatePurchasesResponseWarningsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](InvoicesCreatePurchasesResponseWarningsItemBuilder::code)
    /// - [`message`](InvoicesCreatePurchasesResponseWarningsItemBuilder::message)
    pub fn build(self) -> Result<InvoicesCreatePurchasesResponseWarningsItem, BuildError> {
        Ok(InvoicesCreatePurchasesResponseWarningsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
