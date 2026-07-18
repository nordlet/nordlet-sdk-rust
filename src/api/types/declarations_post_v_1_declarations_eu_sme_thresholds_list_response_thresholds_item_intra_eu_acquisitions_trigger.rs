pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger {
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub note: String,
}

impl PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger {
    pub fn builder(
    ) -> PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder
    {
        <PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder
{
    amount: Option<String>,
    currency: Option<String>,
    note: Option<String>,
}

impl PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder::amount)
    /// - [`currency`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder::currency)
    /// - [`note`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder::note)
    pub fn build(
        self,
    ) -> Result<
        PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger,
        BuildError,
    > {
        Ok(
            PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger {
                amount: self
                    .amount
                    .ok_or_else(|| BuildError::missing_field("amount"))?,
                currency: self
                    .currency
                    .ok_or_else(|| BuildError::missing_field("currency"))?,
                note: self.note.ok_or_else(|| BuildError::missing_field("note"))?,
            },
        )
    }
}
