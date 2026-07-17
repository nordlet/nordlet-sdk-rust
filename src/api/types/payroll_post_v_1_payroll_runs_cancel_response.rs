pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCancelResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1PayrollRunsCancelResponse {
    pub fn builder() -> PostV1PayrollRunsCancelResponseBuilder {
        <PostV1PayrollRunsCancelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCancelResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1PayrollRunsCancelResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCancelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1PayrollRunsCancelResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1PayrollRunsCancelResponse, BuildError> {
        Ok(PostV1PayrollRunsCancelResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
