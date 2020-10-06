use crate::*;
use std::{
    convert::TryFrom,
    ffi::OsStr,
    os::unix::ffi::OsStrExt,
    path::{self, Path},
};
use strum_macros::{AsRefStr, IntoStaticStr};

pub mod iter;
pub mod properties;
pub mod snapshot;
pub use snapshot::*;
pub mod bookmark;
pub use bookmark::*;

pub(crate) use ds::{Dataset as _, RawDataset};

pub(crate) mod ds {
    use crate::{datasets::Dataset as _, zfs_cmd::dmu_objset_type, *};
    use nvpair::NvListT;
    use std::{
        cell::UnsafeCell,
        fmt::Display,
        path::{Path, PathBuf},
    };

    #[derive(Debug)]
    pub struct RawDataset<'a> {
        pub(crate) handle: &'a Handle,
        pub(crate) path: PathBuf,
        /// this uses unsafecell so it can be lazily filled if necessary
        pub(crate) properties: UnsafeCell<Option<NvList>>,
    }

    impl<'a> RawDataset<'a> {
        pub fn new(handle: &'a Handle, path: PathBuf, properties: Option<NvList>) -> Self {
            Self {
                handle,
                path,
                properties: UnsafeCell::new(properties),
            }
        }
    }

    impl Display for RawDataset<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self.path.display(), f)
        }
    }

    impl PartialEq for RawDataset<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.path == other.path
        }
    }

    impl<'a> Dataset for RawDataset<'a> {
        fn handle(&self) -> &Handle {
            &self.handle
        }
        fn raw_path(&self) -> &Path {
            &self.path
        }
        fn reset_raw_properties(&mut self) {
            self.properties = UnsafeCell::new(None);
        }
        fn raw_properties(&self) -> Result<&NvList> {
            {
                let properties = unsafe { &mut *self.properties.get() };
                if properties.is_none() {
                    if let Some(props) = self.handle().properties_nvlist(self.raw_path())? {
                        properties.replace(props);
                    }
                }
            }
            unsafe { self.properties.get().as_ref() }
                .ok_or(Error::NoProperties)?
                .as_ref()
                .ok_or(Error::NoProperties)
        }
    }

    pub trait Dataset {
        fn handle(&self) -> &Handle;
        fn raw_path(&self) -> &Path;
        fn raw_properties(&self) -> Result<&NvList>;
        fn reset_raw_properties(&mut self);

        fn dataset_type(&mut self) -> Result<u32> {
            const TYPE: &CStr = ::cstr::cstr!("type");
            const VALUE: &CStr = ::cstr::cstr!("value");

            if self.raw_path().to_string_lossy().contains('@') {
                return Ok(super::Snapshot::DATASET_TYPE);
            }
            if self.raw_path().to_string_lossy().contains('#') {
                return Ok(super::Bookmark::DATASET_TYPE);
            }
            let dmu_type: u64 = self
                .raw_properties()?
                .lookup::<NvLst>(TYPE)?
                .lookup::<u64>(VALUE)?;
            if dmu_type == dmu_objset_type::DMU_OST_ZVOL as u64 {
                return Ok(super::Volume::DATASET_TYPE);
            }
            if dmu_type == dmu_objset_type::DMU_OST_ZFS as u64 {
                return Ok(super::Filesystem::DATASET_TYPE);
            }
            Err(Error::UnknownDatasetType(dmu_type))
        }
    }
}

pub trait Dataset
where
    Self: ds::Dataset + Sized + std::fmt::Debug,
{
    /// dataset type name
    const NAME: &'static str;

    /// dataset type int
    const DATASET_TYPE: u32;

    /// the separator for the name part of the dataset
    const SEPARATOR: char = '/';

    fn _print_properties(&self) -> Result<()> {
        let props = self.raw_properties()?;
        props._print_props();
        Ok(())
    }

    /// dataset path
    fn path(&self) -> &Path {
        self.raw_path()
    }

    /// the interger value for the dataset type
    fn dataset_type(&self) -> u32 {
        Self::DATASET_TYPE
    }

    /// the interger value for the dataset type
    fn dataset_type_name(&self) -> &'static str {
        Self::NAME
    }

    fn separator(&self) -> char {
        Self::SEPARATOR
    }

    /// dataset name (the part after @ for snapshots or # for bookmarks)
    fn name(&self) -> &OsStr {
        let path = self.path();
        match self.separator() {
            path::MAIN_SEPARATOR => path.file_name().unwrap_or_else(|| path.as_os_str()),
            s => path
                .to_str()
                .and_then(|p| p.rsplit(s).next().map(OsStr::new))
                .unwrap_or_else(|| path.as_os_str()),
        }
    }

    /// pool name
    fn pool(&self) -> &OsStr {
        let path = dbg!(self.path());
        dbg!(dbg!(path.to_str())
            .and_then(|p| dbg!(p.split(&['/', '@', '#'][..]).next()).map(OsStr::new))
            .unwrap_or_else(|| path.as_os_str()))
    }

    /// head name, for filesystems and volumes this is == path, but for snapshots and bookmarks this is the filesystem or volume that is snapshotted
    fn head(&self) -> &Path {
        let path = self.path();
        dbg!(path.to_str()
            .and_then(|p| p.split(&['@', '#'][..]).next().map(Path::new))
            .unwrap_or_else(|| path))
    }

    /// Get a raw property from the dataset
    /// Raw property values are u64 or CStr
    /// This should typically be called by a Property type that converts
    /// the raw value into something useful
    fn raw_property<T: NvPairGetValue>(&self, property: &CStr) -> Result<Option<RawValue<T>>> {
        const SOURCE: &CStr = ::cstr::cstr!("source");
        const VALUE: &CStr = ::cstr::cstr!("value");
        const RECVD: &CStr = ::cstr::cstr!("&recvd");

        let nvlist = self.raw_properties()?;
        let prop_nvl: NvLst = match nvlist.lookup(property) {
            Ok(v) => v,
            Err(Error::NvListError(2)) => return Ok(None),
            Err(e) => return Err(e),
        };
        let prop_nvl_raw = unsafe { prop_nvl.nvlist() };
        let value = unsafe { <T>::nvlist_lookup(prop_nvl_raw, VALUE) }?;
        let source = match nvlist_lookup::<&CStr>(prop_nvl_raw, SOURCE) {
            Err(Error::NvListError(2)) => Source::None,
            Ok(v) if v.to_bytes().is_empty() => Source::Default,
            Ok(v) if v == RECVD => Source::Received,
            Ok(v) if OsStr::from_bytes(v.to_bytes()) == self.path().as_os_str() => Source::Local,
            Ok(_) => Source::Inherited,
            Err(e) => return Err(e),
        };
        Ok(Some(RawValue::new(value, source)))
    }

    /// Get a property from the dataset
    ///
    /// A result of Ok(None) indicates that the property is not set on this dataset.
    /// Some properties have default values that are in effect when a proprety is not set. Use `.unwrap_or_default()` on these properties
    /// to get the default value if the value is not explicitly set.
    fn property<'a, P>(&'a self, property: &P) -> Result<Option<Value<'a, Self, P>>>
    where
        Self: Sized,
        P: DatasetProperty<'a, Self>,
    {
        property.lookup(self)
    }

    /// Set a single property on this dataset
    ///
    /// To set multiple properties atomically, use set_properties
    fn set_property<'a, P: SetableProperty<'a, Self>>(
        &mut self,
        property: &P,
        value: P::Value,
    ) -> Result<()> {
        let mut properties: SetPropertyValues<Self> = SetPropertyValues::default();
        properties.add_value(property, value)?;
        self.set_properties(&properties)
    }

    /// sets multiple properties in one atomic operation
    fn set_properties<'a>(&'a mut self, properties: &SetPropertyValues<Self>) -> Result<()> {
        self.reset_raw_properties();
        self.handle().set_properties(self.path(), &properties.0)
    }

    /// destroy dataset
    fn destroy(self) -> Result<()> {
        self.handle().destroy_dataset(self.path())
    }
}

impl Filesystem<'_> {
    /// gets a child dataset by path
    pub fn get<'a, D>(&'a self, path: &Path) -> Result<D>
    where
        D: Dataset + TryFrom<RawDataset<'a>, Error = Error>,
    {
        self.handle().get(&self.path().join(path))
    }

    /// Get a dataset by path, can be any valid dataset, returned as a Datasets enum
    pub fn get_dataset(&self, path: &Path) -> Result<Datasets> {
        self.get(path)
    }

    /// Get a filesystem by path
    pub fn get_filesystem(&self, path: &Path) -> Result<Filesystem> {
        self.get(path)
    }

    /// Get a volume by path
    pub fn get_volume(&self, path: &Path) -> Result<Volume> {
        self.get(path)
    }

    /// Get a snapshot by path
    pub fn get_snapshot(&self, path: &Path) -> Result<Snapshot> {
        self.get(path)
    }

    /// Get a bookmark by path
    pub fn get_bookmark(&self, path: &Path) -> Result<Bookmark> {
        self.get(path)
    }

    /// Create a child filesystem at path
    pub fn create_filesystem(
        &self,
        path: &Path,
        props: &SetOnCreatePropertyValues<Filesystem>,
    ) -> Result<Filesystem> {
        self.handle()
            .create_filesystem(&self.path().join(path), props)
    }

    /// Create a child volume at path
    pub fn create_volume(
        &self,
        path: &Path,
        size: u64, // todo: byte-unit support
        props: &mut SetOnCreatePropertyValues<Volume>,
    ) -> Result<Volume> {
        self.handle()
            .create_volume(&self.path().join(path), size, props)
    }
}

include!(concat!(env!("ZFSRS_INCLUDE_DIR"), "/datasets.rs"));
