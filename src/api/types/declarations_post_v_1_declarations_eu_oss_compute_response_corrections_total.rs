pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuOssComputeResponseCorrectionsTotal {
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
}

impl PostV1DeclarationsEuOssComputeResponseCorrectionsTotal {
    pub fn builder() -> PostV1DeclarationsEuOssComputeResponseCorrectionsTotalBuilder {
        <PostV1DeclarationsEuOssComputeResponseCorrectionsTotalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuOssComputeResponseCorrectionsTotalBuilder {
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
}

impl PostV1DeclarationsEuOssComputeResponseCorrectionsTotalBuilder {
    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuOssComputeResponseCorrectionsTotal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`taxable_amount`](PostV1DeclarationsEuOssComputeResponseCorrectionsTotalBuilder::taxable_amount)
    /// - [`vat_amount`](PostV1DeclarationsEuOssComputeResponseCorrectionsTotalBuilder::vat_amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuOssComputeResponseCorrectionsTotal, BuildError> {
        Ok(PostV1DeclarationsEuOssComputeResponseCorrectionsTotal {
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
        })
    }
}
