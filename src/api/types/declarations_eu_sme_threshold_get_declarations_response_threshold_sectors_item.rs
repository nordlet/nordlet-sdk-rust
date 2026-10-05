pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem {
    pub fn builder() -> EuSmeThresholdGetDeclarationsResponseThresholdSectorsItemBuilder {
        <EuSmeThresholdGetDeclarationsResponseThresholdSectorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdGetDeclarationsResponseThresholdSectorsItemBuilder {
    label: Option<String>,
    amount: Option<String>,
    note: Option<String>,
}

impl EuSmeThresholdGetDeclarationsResponseThresholdSectorsItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](EuSmeThresholdGetDeclarationsResponseThresholdSectorsItemBuilder::label)
    /// - [`amount`](EuSmeThresholdGetDeclarationsResponseThresholdSectorsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem, BuildError> {
        Ok(EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            note: self.note,
        })
    }
}
