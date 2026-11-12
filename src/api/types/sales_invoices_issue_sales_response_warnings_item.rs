pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesIssueSalesResponseWarningsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl InvoicesIssueSalesResponseWarningsItem {
    pub fn builder() -> InvoicesIssueSalesResponseWarningsItemBuilder {
        <InvoicesIssueSalesResponseWarningsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesIssueSalesResponseWarningsItemBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl InvoicesIssueSalesResponseWarningsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesIssueSalesResponseWarningsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](InvoicesIssueSalesResponseWarningsItemBuilder::code)
    /// - [`message`](InvoicesIssueSalesResponseWarningsItemBuilder::message)
    pub fn build(self) -> Result<InvoicesIssueSalesResponseWarningsItem, BuildError> {
        Ok(InvoicesIssueSalesResponseWarningsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
