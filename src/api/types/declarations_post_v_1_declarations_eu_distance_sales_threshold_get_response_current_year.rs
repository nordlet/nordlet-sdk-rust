pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear {
    pub fn builder() -> PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYearBuilder {
        <PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYearBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYearBuilder {
    year: Option<i64>,
    total_amount: Option<String>,
    documents: Option<i64>,
}

impl PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYearBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn total_amount(mut self, value: impl Into<String>) -> Self {
        self.total_amount = Some(value.into());
        self
    }

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYearBuilder::year)
    /// - [`total_amount`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYearBuilder::total_amount)
    /// - [`documents`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYearBuilder::documents)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear, BuildError> {
        Ok(
            PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear {
                year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
                total_amount: self
                    .total_amount
                    .ok_or_else(|| BuildError::missing_field("total_amount"))?,
                documents: self
                    .documents
                    .ok_or_else(|| BuildError::missing_field("documents"))?,
            },
        )
    }
}
