pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsAdvanceReconciliationResponseRowsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub opening: String,
    #[serde(default)]
    pub issued: String,
    #[serde(default)]
    pub returned: String,
    #[serde(default)]
    pub closing: String,
}

impl PostV1ReportsAdvanceReconciliationResponseRowsItem {
    pub fn builder() -> PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder {
        <PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder {
    employee_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    opening: Option<String>,
    issued: Option<String>,
    returned: Option<String>,
    closing: Option<String>,
}

impl PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn opening(mut self, value: impl Into<String>) -> Self {
        self.opening = Some(value.into());
        self
    }

    pub fn issued(mut self, value: impl Into<String>) -> Self {
        self.issued = Some(value.into());
        self
    }

    pub fn returned(mut self, value: impl Into<String>) -> Self {
        self.returned = Some(value.into());
        self
    }

    pub fn closing(mut self, value: impl Into<String>) -> Self {
        self.closing = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsAdvanceReconciliationResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder::employee_id)
    /// - [`first_name`](PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder::first_name)
    /// - [`last_name`](PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder::last_name)
    /// - [`opening`](PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder::opening)
    /// - [`issued`](PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder::issued)
    /// - [`returned`](PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder::returned)
    /// - [`closing`](PostV1ReportsAdvanceReconciliationResponseRowsItemBuilder::closing)
    pub fn build(self) -> Result<PostV1ReportsAdvanceReconciliationResponseRowsItem, BuildError> {
        Ok(PostV1ReportsAdvanceReconciliationResponseRowsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            opening: self
                .opening
                .ok_or_else(|| BuildError::missing_field("opening"))?,
            issued: self
                .issued
                .ok_or_else(|| BuildError::missing_field("issued"))?,
            returned: self
                .returned
                .ok_or_else(|| BuildError::missing_field("returned"))?,
            closing: self
                .closing
                .ok_or_else(|| BuildError::missing_field("closing"))?,
        })
    }
}
