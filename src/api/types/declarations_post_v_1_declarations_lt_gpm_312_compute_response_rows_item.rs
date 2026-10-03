pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm312ComputeResponseRowsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "personalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_code: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(rename = "paymentCode")]
    #[serde(default)]
    pub payment_code: String,
    #[serde(rename = "paidAmount")]
    #[serde(default)]
    pub paid_amount: String,
    #[serde(rename = "gpmWithheld")]
    #[serde(default)]
    pub gpm_withheld: String,
}

impl PostV1DeclarationsLtGpm312ComputeResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder {
        <PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder {
    employee_id: Option<String>,
    personal_code: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    payment_code: Option<String>,
    paid_amount: Option<String>,
    gpm_withheld: Option<String>,
}

impl PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn personal_code(mut self, value: impl Into<String>) -> Self {
        self.personal_code = Some(value.into());
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

    pub fn payment_code(mut self, value: impl Into<String>) -> Self {
        self.payment_code = Some(value.into());
        self
    }

    pub fn paid_amount(mut self, value: impl Into<String>) -> Self {
        self.paid_amount = Some(value.into());
        self
    }

    pub fn gpm_withheld(mut self, value: impl Into<String>) -> Self {
        self.gpm_withheld = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm312ComputeResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder::employee_id)
    /// - [`first_name`](PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder::first_name)
    /// - [`last_name`](PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder::last_name)
    /// - [`payment_code`](PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder::payment_code)
    /// - [`paid_amount`](PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder::paid_amount)
    /// - [`gpm_withheld`](PostV1DeclarationsLtGpm312ComputeResponseRowsItemBuilder::gpm_withheld)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm312ComputeResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsLtGpm312ComputeResponseRowsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            personal_code: self.personal_code,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            payment_code: self
                .payment_code
                .ok_or_else(|| BuildError::missing_field("payment_code"))?,
            paid_amount: self
                .paid_amount
                .ok_or_else(|| BuildError::missing_field("paid_amount"))?,
            gpm_withheld: self
                .gpm_withheld
                .ok_or_else(|| BuildError::missing_field("gpm_withheld"))?,
        })
    }
}
