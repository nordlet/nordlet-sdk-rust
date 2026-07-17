pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsFinancialStatementsResponseEquityChangesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub opening: String,
    #[serde(default)]
    pub increase: String,
    #[serde(default)]
    pub decrease: String,
    #[serde(default)]
    pub closing: String,
}

impl PostV1ReportsFinancialStatementsResponseEquityChangesItem {
    pub fn builder() -> PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder {
        <PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder {
    code: Option<String>,
    name: Option<String>,
    opening: Option<String>,
    increase: Option<String>,
    decrease: Option<String>,
    closing: Option<String>,
}

impl PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn opening(mut self, value: impl Into<String>) -> Self {
        self.opening = Some(value.into());
        self
    }

    pub fn increase(mut self, value: impl Into<String>) -> Self {
        self.increase = Some(value.into());
        self
    }

    pub fn decrease(mut self, value: impl Into<String>) -> Self {
        self.decrease = Some(value.into());
        self
    }

    pub fn closing(mut self, value: impl Into<String>) -> Self {
        self.closing = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsFinancialStatementsResponseEquityChangesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder::code)
    /// - [`name`](PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder::name)
    /// - [`opening`](PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder::opening)
    /// - [`increase`](PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder::increase)
    /// - [`decrease`](PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder::decrease)
    /// - [`closing`](PostV1ReportsFinancialStatementsResponseEquityChangesItemBuilder::closing)
    pub fn build(
        self,
    ) -> Result<PostV1ReportsFinancialStatementsResponseEquityChangesItem, BuildError> {
        Ok(PostV1ReportsFinancialStatementsResponseEquityChangesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            opening: self
                .opening
                .ok_or_else(|| BuildError::missing_field("opening"))?,
            increase: self
                .increase
                .ok_or_else(|| BuildError::missing_field("increase"))?,
            decrease: self
                .decrease
                .ok_or_else(|| BuildError::missing_field("decrease"))?,
            closing: self
                .closing
                .ok_or_else(|| BuildError::missing_field("closing"))?,
        })
    }
}
