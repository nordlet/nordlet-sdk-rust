pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm312ComputeResponseTotals {
    #[serde(rename = "paidAmount")]
    #[serde(default)]
    pub paid_amount: String,
    #[serde(rename = "gpmWithheld")]
    #[serde(default)]
    pub gpm_withheld: String,
    #[serde(default)]
    pub persons: i64,
}

impl PostV1DeclarationsLtGpm312ComputeResponseTotals {
    pub fn builder() -> PostV1DeclarationsLtGpm312ComputeResponseTotalsBuilder {
        <PostV1DeclarationsLtGpm312ComputeResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm312ComputeResponseTotalsBuilder {
    paid_amount: Option<String>,
    gpm_withheld: Option<String>,
    persons: Option<i64>,
}

impl PostV1DeclarationsLtGpm312ComputeResponseTotalsBuilder {
    pub fn paid_amount(mut self, value: impl Into<String>) -> Self {
        self.paid_amount = Some(value.into());
        self
    }

    pub fn gpm_withheld(mut self, value: impl Into<String>) -> Self {
        self.gpm_withheld = Some(value.into());
        self
    }

    pub fn persons(mut self, value: i64) -> Self {
        self.persons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm312ComputeResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paid_amount`](PostV1DeclarationsLtGpm312ComputeResponseTotalsBuilder::paid_amount)
    /// - [`gpm_withheld`](PostV1DeclarationsLtGpm312ComputeResponseTotalsBuilder::gpm_withheld)
    /// - [`persons`](PostV1DeclarationsLtGpm312ComputeResponseTotalsBuilder::persons)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm312ComputeResponseTotals, BuildError> {
        Ok(PostV1DeclarationsLtGpm312ComputeResponseTotals {
            paid_amount: self
                .paid_amount
                .ok_or_else(|| BuildError::missing_field("paid_amount"))?,
            gpm_withheld: self
                .gpm_withheld
                .ok_or_else(|| BuildError::missing_field("gpm_withheld"))?,
            persons: self
                .persons
                .ok_or_else(|| BuildError::missing_field("persons"))?,
        })
    }
}
