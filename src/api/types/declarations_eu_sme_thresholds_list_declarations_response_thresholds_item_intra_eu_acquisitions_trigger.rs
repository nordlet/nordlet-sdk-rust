pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger {
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub note: String,
}

impl EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger {
    pub fn builder(
    ) -> EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder
    {
        <EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder {
    amount: Option<String>,
    currency: Option<String>,
    note: Option<String>,
}

impl EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder {
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

    /// Consumes the builder and constructs a [`EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder::amount)
    /// - [`currency`](EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder::currency)
    /// - [`note`](EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTriggerBuilder::note)
    pub fn build(
        self,
    ) -> Result<
        EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger,
        BuildError,
    > {
        Ok(
            EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger {
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
