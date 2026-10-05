pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PositionsCreateHrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, PositionsCreateHrRequestTranslationsValue>>,
}

impl PositionsCreateHrRequest {
    pub fn builder() -> PositionsCreateHrRequestBuilder {
        <PositionsCreateHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsCreateHrRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    translations: Option<HashMap<String, PositionsCreateHrRequestTranslationsValue>>,
}

impl PositionsCreateHrRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, PositionsCreateHrRequestTranslationsValue>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PositionsCreateHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PositionsCreateHrRequestBuilder::name)
    pub fn build(self) -> Result<PositionsCreateHrRequest, BuildError> {
        Ok(PositionsCreateHrRequest {
            code: self.code,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            translations: self.translations,
        })
    }
}
