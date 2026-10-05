pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem {
    pub fn builder() -> EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItemBuilder {
        <EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItemBuilder {
    label: Option<String>,
    amount: Option<String>,
    note: Option<String>,
}

impl EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItemBuilder {
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

    /// Consumes the builder and constructs a [`EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItemBuilder::label)
    /// - [`amount`](EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem, BuildError> {
        Ok(
            EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem {
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
                amount: self
                    .amount
                    .ok_or_else(|| BuildError::missing_field("amount"))?,
                note: self.note,
            },
        )
    }
}
