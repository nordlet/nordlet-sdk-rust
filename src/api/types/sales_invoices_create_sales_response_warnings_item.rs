pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCreateSalesResponseWarningsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl InvoicesCreateSalesResponseWarningsItem {
    pub fn builder() -> InvoicesCreateSalesResponseWarningsItemBuilder {
        <InvoicesCreateSalesResponseWarningsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCreateSalesResponseWarningsItemBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl InvoicesCreateSalesResponseWarningsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCreateSalesResponseWarningsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](InvoicesCreateSalesResponseWarningsItemBuilder::code)
    /// - [`message`](InvoicesCreateSalesResponseWarningsItemBuilder::message)
    pub fn build(self) -> Result<InvoicesCreateSalesResponseWarningsItem, BuildError> {
        Ok(InvoicesCreateSalesResponseWarningsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
