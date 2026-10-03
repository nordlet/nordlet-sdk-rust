pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm312ComputeResponsePayoutTo {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsLtGpm312ComputeResponsePayoutTo {
    pub fn builder() -> PostV1DeclarationsLtGpm312ComputeResponsePayoutToBuilder {
        <PostV1DeclarationsLtGpm312ComputeResponsePayoutToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm312ComputeResponsePayoutToBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsLtGpm312ComputeResponsePayoutToBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm312ComputeResponsePayoutTo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtGpm312ComputeResponsePayoutToBuilder::year)
    /// - [`month`](PostV1DeclarationsLtGpm312ComputeResponsePayoutToBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm312ComputeResponsePayoutTo, BuildError> {
        Ok(PostV1DeclarationsLtGpm312ComputeResponsePayoutTo {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
