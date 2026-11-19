pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem {
    #[serde(default)]
    pub shares: i64,
    #[serde(rename = "parValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_value: Option<String>,
}

impl UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem {
    pub fn builder() -> UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItemBuilder {
        <UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItemBuilder {
    shares: Option<i64>,
    par_value: Option<String>,
}

impl UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItemBuilder {
    pub fn shares(mut self, value: i64) -> Self {
        self.shares = Some(value);
        self
    }

    pub fn par_value(mut self, value: impl Into<String>) -> Self {
        self.par_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`shares`](UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItemBuilder::shares)
    pub fn build(
        self,
    ) -> Result<UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem, BuildError> {
        Ok(
            UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem {
                shares: self
                    .shares
                    .ok_or_else(|| BuildError::missing_field("shares"))?,
                par_value: self.par_value,
            },
        )
    }
}
