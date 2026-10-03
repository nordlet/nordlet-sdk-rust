pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkMagGenerateResponseCounts {
    #[serde(default)]
    pub pz: i64,
    #[serde(default)]
    pub pw: i64,
    #[serde(default)]
    pub wz: i64,
    #[serde(default)]
    pub rw: i64,
    #[serde(default)]
    pub rows: i64,
}

impl PostV1DeclarationsPlJpkMagGenerateResponseCounts {
    pub fn builder() -> PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder {
        <PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder {
    pz: Option<i64>,
    pw: Option<i64>,
    wz: Option<i64>,
    rw: Option<i64>,
    rows: Option<i64>,
}

impl PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder {
    pub fn pz(mut self, value: i64) -> Self {
        self.pz = Some(value);
        self
    }

    pub fn pw(mut self, value: i64) -> Self {
        self.pw = Some(value);
        self
    }

    pub fn wz(mut self, value: i64) -> Self {
        self.wz = Some(value);
        self
    }

    pub fn rw(mut self, value: i64) -> Self {
        self.rw = Some(value);
        self
    }

    pub fn rows(mut self, value: i64) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkMagGenerateResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pz`](PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder::pz)
    /// - [`pw`](PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder::pw)
    /// - [`wz`](PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder::wz)
    /// - [`rw`](PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder::rw)
    /// - [`rows`](PostV1DeclarationsPlJpkMagGenerateResponseCountsBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkMagGenerateResponseCounts, BuildError> {
        Ok(PostV1DeclarationsPlJpkMagGenerateResponseCounts {
            pz: self.pz.ok_or_else(|| BuildError::missing_field("pz"))?,
            pw: self.pw.ok_or_else(|| BuildError::missing_field("pw"))?,
            wz: self.wz.ok_or_else(|| BuildError::missing_field("wz"))?,
            rw: self.rw.ok_or_else(|| BuildError::missing_field("rw"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
