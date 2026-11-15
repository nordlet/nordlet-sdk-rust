pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsCorrectionPreviewDeclarationsResponseChangesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
}

impl SubmissionsCorrectionPreviewDeclarationsResponseChangesItem {
    pub fn builder() -> SubmissionsCorrectionPreviewDeclarationsResponseChangesItemBuilder {
        <SubmissionsCorrectionPreviewDeclarationsResponseChangesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsCorrectionPreviewDeclarationsResponseChangesItemBuilder {
    field: Option<String>,
    line: Option<i64>,
    filed: Option<String>,
    current: Option<String>,
}

impl SubmissionsCorrectionPreviewDeclarationsResponseChangesItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn filed(mut self, value: impl Into<String>) -> Self {
        self.filed = Some(value.into());
        self
    }

    pub fn current(mut self, value: impl Into<String>) -> Self {
        self.current = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsCorrectionPreviewDeclarationsResponseChangesItem`].
    pub fn build(
        self,
    ) -> Result<SubmissionsCorrectionPreviewDeclarationsResponseChangesItem, BuildError> {
        Ok(
            SubmissionsCorrectionPreviewDeclarationsResponseChangesItem {
                field: self.field,
                line: self.line,
                filed: self.filed,
                current: self.current,
            },
        )
    }
}
