pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsW2ListDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl UsW2ListDeclarationsRequest {
    pub fn builder() -> UsW2ListDeclarationsRequestBuilder {
        <UsW2ListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsW2ListDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl UsW2ListDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsW2ListDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](UsW2ListDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<UsW2ListDeclarationsRequest, BuildError> {
        Ok(UsW2ListDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
