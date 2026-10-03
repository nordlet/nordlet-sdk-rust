pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom {
    pub fn builder() -> PostV1DeclarationsLtGpm312ComputeResponsePayoutFromBuilder {
        <PostV1DeclarationsLtGpm312ComputeResponsePayoutFromBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm312ComputeResponsePayoutFromBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsLtGpm312ComputeResponsePayoutFromBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtGpm312ComputeResponsePayoutFromBuilder::year)
    /// - [`month`](PostV1DeclarationsLtGpm312ComputeResponsePayoutFromBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom, BuildError> {
        Ok(PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
