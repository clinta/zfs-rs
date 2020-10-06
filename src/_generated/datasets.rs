#[derive(AsRefStr, IntoStaticStr, PartialEq, Debug)]
pub enum Datasets<'a> {
    #[strum(to_string = "filesystem")]
    Filesystem(Filesystem<'a>),
    #[strum(to_string = "snapshot")]
    Snapshot(Snapshot<'a>),
    #[strum(to_string = "volume")]
    Volume(Volume<'a>),
    #[strum(to_string = "pool")]
    Pool(Pool<'a>),
    #[strum(to_string = "bookmark")]
    Bookmark(Bookmark<'a>),
}
impl ds::Dataset for Datasets<'_> {
    fn handle(&self) -> &Handle {
        match self {
            Self::Filesystem(v) => v.handle(),
            Self::Snapshot(v) => v.handle(),
            Self::Volume(v) => v.handle(),
            Self::Pool(v) => v.handle(),
            Self::Bookmark(v) => v.handle(),
        }
    }
    fn raw_path(&self) -> &Path {
        match self {
            Self::Filesystem(v) => v.raw_path(),
            Self::Snapshot(v) => v.raw_path(),
            Self::Volume(v) => v.raw_path(),
            Self::Pool(v) => v.raw_path(),
            Self::Bookmark(v) => v.raw_path(),
        }
    }
    fn raw_properties(&self) -> Result<&NvList> {
        match self {
            Self::Filesystem(v) => v.raw_properties(),
            Self::Snapshot(v) => v.raw_properties(),
            Self::Volume(v) => v.raw_properties(),
            Self::Pool(v) => v.raw_properties(),
            Self::Bookmark(v) => v.raw_properties(),
        }
    }
    fn reset_raw_properties(&mut self) {
        match self {
            Self::Filesystem(v) => v.reset_raw_properties(),
            Self::Snapshot(v) => v.reset_raw_properties(),
            Self::Volume(v) => v.reset_raw_properties(),
            Self::Pool(v) => v.reset_raw_properties(),
            Self::Bookmark(v) => v.reset_raw_properties(),
        }
    }
}
impl<'a> Dataset for Datasets<'a> {
    const NAME: &'static str = "filesystem|snapshot|volume|pool|bookmark";
    const SEPARATOR: char = '/';
    const DATASET_TYPE: u32 = std::u32::MAX;
    fn path(&self) -> &Path {
        match self {
            Self::Filesystem(v) => v.path(),
            Self::Snapshot(v) => v.path(),
            Self::Volume(v) => v.path(),
            Self::Pool(v) => v.path(),
            Self::Bookmark(v) => v.path(),
        }
    }
    fn separator(&self) -> char {
        match self {
            Self::Filesystem(v) => v.separator(),
            Self::Snapshot(v) => v.separator(),
            Self::Volume(v) => v.separator(),
            Self::Pool(v) => v.separator(),
            Self::Bookmark(v) => v.separator(),
        }
    }
    fn dataset_type(&self) -> u32 {
        match self {
            Self::Filesystem(v) => v.dataset_type(),
            Self::Snapshot(v) => v.dataset_type(),
            Self::Volume(v) => v.dataset_type(),
            Self::Pool(v) => v.dataset_type(),
            Self::Bookmark(v) => v.dataset_type(),
        }
    }
    fn dataset_type_name(&self) -> &'static str {
        match self {
            Self::Filesystem(v) => v.dataset_type_name(),
            Self::Snapshot(v) => v.dataset_type_name(),
            Self::Volume(v) => v.dataset_type_name(),
            Self::Pool(v) => v.dataset_type_name(),
            Self::Bookmark(v) => v.dataset_type_name(),
        }
    }
}
impl<'a> TryFrom<RawDataset<'a>> for Datasets<'a> {
    type Error = Error;
    fn try_from(mut rds: RawDataset<'a>) -> Result<Self> {
        match rds.dataset_type()? {
            Filesystem::DATASET_TYPE => Self::Filesystem(Filesystem { raw: rds }).apply(Ok),
            Snapshot::DATASET_TYPE => Self::Snapshot(Snapshot { raw: rds }).apply(Ok),
            Volume::DATASET_TYPE => Self::Volume(Volume { raw: rds }).apply(Ok),
            Pool::DATASET_TYPE => Self::Pool(Pool { raw: rds }).apply(Ok),
            Bookmark::DATASET_TYPE => Self::Bookmark(Bookmark { raw: rds }).apply(Ok),
            v => Err(Error::UnknownDatasetType(v.into())),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Filesystem<'a> {
    pub(crate) raw: RawDataset<'a>,
}
impl ds::Dataset for Filesystem<'_> {
    fn handle(&self) -> &Handle {
        &self.raw.handle
    }
    fn raw_path(&self) -> &Path {
        &self.raw.path
    }
    fn raw_properties(&self) -> Result<&NvList> {
        self.raw.raw_properties()
    }
    fn reset_raw_properties(&mut self) {
        self.raw.reset_raw_properties()
    }
}
impl<'a> Dataset for Filesystem<'a> {
    const NAME: &'static str = "filesystem";
    const SEPARATOR: char = '/';
    const DATASET_TYPE: u32 = 1u32;
}
impl<'a> From<Filesystem<'a>> for Datasets<'a> {
    fn from(ds: Filesystem<'a>) -> Self {
        Self::Filesystem(ds)
    }
}
impl<'a> TryFrom<Datasets<'a>> for Filesystem<'a> {
    type Error = Error;
    fn try_from(ads: Datasets<'a>) -> Result<Self> {
        match ads {
            Datasets::Filesystem(ds) => Ok(ds),
            _ => Err(Error::WrongDatasetType(ads.into())),
        }
    }
}
impl<'a> TryFrom<RawDataset<'a>> for Filesystem<'a> {
    type Error = Error;
    fn try_from(rds: RawDataset<'a>) -> Result<Self> {
        Datasets::try_from(rds)?.try_into()
    }
}
#[derive(Debug, PartialEq)]
pub struct Snapshot<'a> {
    pub(crate) raw: RawDataset<'a>,
}
impl ds::Dataset for Snapshot<'_> {
    fn handle(&self) -> &Handle {
        &self.raw.handle
    }
    fn raw_path(&self) -> &Path {
        &self.raw.path
    }
    fn raw_properties(&self) -> Result<&NvList> {
        self.raw.raw_properties()
    }
    fn reset_raw_properties(&mut self) {
        self.raw.reset_raw_properties()
    }
}
impl<'a> Dataset for Snapshot<'a> {
    const NAME: &'static str = "snapshot";
    const SEPARATOR: char = '@';
    const DATASET_TYPE: u32 = 2u32;
}
impl<'a> From<Snapshot<'a>> for Datasets<'a> {
    fn from(ds: Snapshot<'a>) -> Self {
        Self::Snapshot(ds)
    }
}
impl<'a> TryFrom<Datasets<'a>> for Snapshot<'a> {
    type Error = Error;
    fn try_from(ads: Datasets<'a>) -> Result<Self> {
        match ads {
            Datasets::Snapshot(ds) => Ok(ds),
            _ => Err(Error::WrongDatasetType(ads.into())),
        }
    }
}
impl<'a> TryFrom<RawDataset<'a>> for Snapshot<'a> {
    type Error = Error;
    fn try_from(rds: RawDataset<'a>) -> Result<Self> {
        Datasets::try_from(rds)?.try_into()
    }
}
#[derive(Debug, PartialEq)]
pub struct Volume<'a> {
    pub(crate) raw: RawDataset<'a>,
}
impl ds::Dataset for Volume<'_> {
    fn handle(&self) -> &Handle {
        &self.raw.handle
    }
    fn raw_path(&self) -> &Path {
        &self.raw.path
    }
    fn raw_properties(&self) -> Result<&NvList> {
        self.raw.raw_properties()
    }
    fn reset_raw_properties(&mut self) {
        self.raw.reset_raw_properties()
    }
}
impl<'a> Dataset for Volume<'a> {
    const NAME: &'static str = "volume";
    const SEPARATOR: char = '/';
    const DATASET_TYPE: u32 = 4u32;
}
impl<'a> From<Volume<'a>> for Datasets<'a> {
    fn from(ds: Volume<'a>) -> Self {
        Self::Volume(ds)
    }
}
impl<'a> TryFrom<Datasets<'a>> for Volume<'a> {
    type Error = Error;
    fn try_from(ads: Datasets<'a>) -> Result<Self> {
        match ads {
            Datasets::Volume(ds) => Ok(ds),
            _ => Err(Error::WrongDatasetType(ads.into())),
        }
    }
}
impl<'a> TryFrom<RawDataset<'a>> for Volume<'a> {
    type Error = Error;
    fn try_from(rds: RawDataset<'a>) -> Result<Self> {
        Datasets::try_from(rds)?.try_into()
    }
}
#[derive(Debug, PartialEq)]
pub struct Pool<'a> {
    pub(crate) raw: RawDataset<'a>,
}
impl ds::Dataset for Pool<'_> {
    fn handle(&self) -> &Handle {
        &self.raw.handle
    }
    fn raw_path(&self) -> &Path {
        &self.raw.path
    }
    fn raw_properties(&self) -> Result<&NvList> {
        self.raw.raw_properties()
    }
    fn reset_raw_properties(&mut self) {
        self.raw.reset_raw_properties()
    }
}
impl<'a> Dataset for Pool<'a> {
    const NAME: &'static str = "pool";
    const SEPARATOR: char = '/';
    const DATASET_TYPE: u32 = 8u32;
}
impl<'a> From<Pool<'a>> for Datasets<'a> {
    fn from(ds: Pool<'a>) -> Self {
        Self::Pool(ds)
    }
}
impl<'a> TryFrom<Datasets<'a>> for Pool<'a> {
    type Error = Error;
    fn try_from(ads: Datasets<'a>) -> Result<Self> {
        match ads {
            Datasets::Pool(ds) => Ok(ds),
            _ => Err(Error::WrongDatasetType(ads.into())),
        }
    }
}
impl<'a> TryFrom<RawDataset<'a>> for Pool<'a> {
    type Error = Error;
    fn try_from(rds: RawDataset<'a>) -> Result<Self> {
        Datasets::try_from(rds)?.try_into()
    }
}
#[derive(Debug, PartialEq)]
pub struct Bookmark<'a> {
    pub(crate) raw: RawDataset<'a>,
}
impl ds::Dataset for Bookmark<'_> {
    fn handle(&self) -> &Handle {
        &self.raw.handle
    }
    fn raw_path(&self) -> &Path {
        &self.raw.path
    }
    fn raw_properties(&self) -> Result<&NvList> {
        self.raw.raw_properties()
    }
    fn reset_raw_properties(&mut self) {
        self.raw.reset_raw_properties()
    }
}
impl<'a> Dataset for Bookmark<'a> {
    const NAME: &'static str = "bookmark";
    const SEPARATOR: char = '#';
    const DATASET_TYPE: u32 = 16u32;
}
impl<'a> From<Bookmark<'a>> for Datasets<'a> {
    fn from(ds: Bookmark<'a>) -> Self {
        Self::Bookmark(ds)
    }
}
impl<'a> TryFrom<Datasets<'a>> for Bookmark<'a> {
    type Error = Error;
    fn try_from(ads: Datasets<'a>) -> Result<Self> {
        match ads {
            Datasets::Bookmark(ds) => Ok(ds),
            _ => Err(Error::WrongDatasetType(ads.into())),
        }
    }
}
impl<'a> TryFrom<RawDataset<'a>> for Bookmark<'a> {
    type Error = Error;
    fn try_from(rds: RawDataset<'a>) -> Result<Self> {
        Datasets::try_from(rds)?.try_into()
    }
}
