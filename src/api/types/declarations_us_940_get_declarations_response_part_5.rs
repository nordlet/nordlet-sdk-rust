pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Us940GetDeclarationsResponsePart5 {
    #[serde(default)]
    pub line16: Vec<String>,
    #[serde(default)]
    pub line17: String,
}

impl Us940GetDeclarationsResponsePart5 {
    pub fn builder() -> Us940GetDeclarationsResponsePart5Builder {
        <Us940GetDeclarationsResponsePart5Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us940GetDeclarationsResponsePart5Builder {
    line16: Option<Vec<String>>,
    line17: Option<String>,
}

impl Us940GetDeclarationsResponsePart5Builder {
    pub fn line16(mut self, value: Vec<String>) -> Self {
        self.line16 = Some(value);
        self
    }

    pub fn line17(mut self, value: impl Into<String>) -> Self {
        self.line17 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Us940GetDeclarationsResponsePart5`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line16`](Us940GetDeclarationsResponsePart5Builder::line16)
    /// - [`line17`](Us940GetDeclarationsResponsePart5Builder::line17)
    pub fn build(self) -> Result<Us940GetDeclarationsResponsePart5, BuildError> {
        Ok(Us940GetDeclarationsResponsePart5 {
            line16: self
                .line16
                .ok_or_else(|| BuildError::missing_field("line16"))?,
            line17: self
                .line17
                .ok_or_else(|| BuildError::missing_field("line17"))?,
        })
    }
}
