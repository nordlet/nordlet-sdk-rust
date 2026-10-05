pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkMagGenerateDeclarationsResponseCounts {
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

impl PlJpkMagGenerateDeclarationsResponseCounts {
    pub fn builder() -> PlJpkMagGenerateDeclarationsResponseCountsBuilder {
        <PlJpkMagGenerateDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkMagGenerateDeclarationsResponseCountsBuilder {
    pz: Option<i64>,
    pw: Option<i64>,
    wz: Option<i64>,
    rw: Option<i64>,
    rows: Option<i64>,
}

impl PlJpkMagGenerateDeclarationsResponseCountsBuilder {
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

    /// Consumes the builder and constructs a [`PlJpkMagGenerateDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pz`](PlJpkMagGenerateDeclarationsResponseCountsBuilder::pz)
    /// - [`pw`](PlJpkMagGenerateDeclarationsResponseCountsBuilder::pw)
    /// - [`wz`](PlJpkMagGenerateDeclarationsResponseCountsBuilder::wz)
    /// - [`rw`](PlJpkMagGenerateDeclarationsResponseCountsBuilder::rw)
    /// - [`rows`](PlJpkMagGenerateDeclarationsResponseCountsBuilder::rows)
    pub fn build(self) -> Result<PlJpkMagGenerateDeclarationsResponseCounts, BuildError> {
        Ok(PlJpkMagGenerateDeclarationsResponseCounts {
            pz: self.pz.ok_or_else(|| BuildError::missing_field("pz"))?,
            pw: self.pw.ok_or_else(|| BuildError::missing_field("pw"))?,
            wz: self.wz.ok_or_else(|| BuildError::missing_field("wz"))?,
            rw: self.rw.ok_or_else(|| BuildError::missing_field("rw"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
