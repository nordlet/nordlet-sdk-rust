pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItemBuilder
    {
        <PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItemBuilder {
    label: Option<String>,
    amount: Option<String>,
    note: Option<String>,
}

impl PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItemBuilder::label)
    /// - [`amount`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem, BuildError>
    {
        Ok(
            PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem {
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
