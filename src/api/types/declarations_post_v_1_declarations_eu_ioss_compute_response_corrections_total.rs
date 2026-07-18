pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuIossComputeResponseCorrectionsTotal {
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
}

impl PostV1DeclarationsEuIossComputeResponseCorrectionsTotal {
    pub fn builder() -> PostV1DeclarationsEuIossComputeResponseCorrectionsTotalBuilder {
        <PostV1DeclarationsEuIossComputeResponseCorrectionsTotalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuIossComputeResponseCorrectionsTotalBuilder {
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
}

impl PostV1DeclarationsEuIossComputeResponseCorrectionsTotalBuilder {
    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuIossComputeResponseCorrectionsTotal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`taxable_amount`](PostV1DeclarationsEuIossComputeResponseCorrectionsTotalBuilder::taxable_amount)
    /// - [`vat_amount`](PostV1DeclarationsEuIossComputeResponseCorrectionsTotalBuilder::vat_amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuIossComputeResponseCorrectionsTotal, BuildError> {
        Ok(PostV1DeclarationsEuIossComputeResponseCorrectionsTotal {
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
        })
    }
}
