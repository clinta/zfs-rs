use apply::*;
use datasets::*;
use properties::*;
use std::{
    convert::TryFrom, convert::TryInto, ffi::CStr, ffi::CString, fs::File, fs::OpenOptions,
    os::unix::io::AsRawFd, path::Path, path::PathBuf,
};
use zfs_cmd::{ioctl_fn, ZfsCmd};

pub use datasets::*;
pub use errors::*;
pub(crate) use nvpair::*;

pub mod datasets;
mod errors;
mod nvpair;
mod zfs_cmd;

pub(crate) extern crate self as zfs_rs;

/// A handle is an open session to /dev/zfs
#[derive(Debug)]
pub struct Handle(File);

impl Handle {
    /// Get a new handle to /dev/zfs
    pub fn new() -> std::io::Result<Self> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/zfs")
            .map(Self)
    }

    // run_cmd would seem to require a mutable ref, but it does not, raw_fd does not require it
    // and libzfs_core does not wrap calling ioctls in a mutex, so I don't think it's required
    fn run_cmd<'a>(
        &self,
        zfs_cmd: &'a mut ZfsCmd,
        request: zfs_cmd::RequestFn,
    ) -> Result<&'a mut ZfsCmd> {
        zfs_cmd.run(self.0.as_raw_fd(), request)
    }

    /// Get an iterator over all the root filesystems
    pub fn root_filesystems(&self) -> Result<impl Iterator<Item = Result<Filesystem>>> {
        // This bypasses the normal constructor via Datasets, because root datasets can only be filesystems
        // this avoids calling out to get propreties if properties are not needed
        self.run_cmd(&mut ZfsCmd::default(), ioctl_fn::pool_configs)?
            .dst()
            .apply(NvListIterator::from)
            .map(move |mut x| {
                datasets::Filesystem {
                    raw: RawDataset::new(&self, PathBuf::from(x.name()?), None),
                }
                .apply(Ok)
            })
            .apply(Ok)
    }

    /// Get the properties nvlist for a dataset path
    pub(crate) fn properties_nvlist(&self, path: &Path) -> Result<Option<NvList>> {
        self.run_cmd(
            &mut ZfsCmd::default().set_name(path),
            ioctl_fn::objset_stats,
        )?
        .dst()
        .apply(Ok)
    }

    /// Get a dataset of type D by path
    pub fn get<'a, D>(&'a self, path: &Path) -> Result<D>
    where
        D: Dataset + TryFrom<RawDataset<'a>, Error = Error>,
    {
        let properties = self.properties_nvlist(path)?;
        RawDataset::new(&self, path.into(), properties).try_into()
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

    /// Iterate over the child datasets of a path
    pub fn iter_children<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl Iterator<Item = Result<Datasets<'a>>> {
        datasets::iter::ChildIterator::new(self, path)
    }

    /// Recursively iterate over child datasets of a path
    pub fn iter_children_recursive<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl Iterator<Item = Result<Datasets<'a>>> {
        datasets::iter::ChildIteratorRecursive::new(self, path.into())
    }

    /// Create a dataset of dataset_type
    fn create_dataset(
        &self,
        path: &Path,
        dataset_type: zfs_cmd::dmu_objset_type,
        props: &NvList,
    ) -> Result<()> {
        const TYPE: &CStr = ::cstr::cstr!("type");
        const PROPS: &CStr = ::cstr::cstr!("props");
        let mut args = NvList::default();
        args.add(TYPE, dataset_type as i32)?;
        args.add(PROPS, props)?;
        self.run_cmd(
            &mut ZfsCmd::default().set_name(path).set_src(&args),
            ioctl_fn::create,
        )?;
        Ok(())
    }

    /// Create a filesystem at path
    pub fn create_filesystem(
        &self,
        path: &Path,
        props: &SetOnCreatePropertyValues<Filesystem>,
    ) -> Result<Filesystem> {
        self.create_dataset(path, zfs_cmd::dmu_objset_type::DMU_OST_ZFS, &props.0)?;
        datasets::Filesystem {
            raw: RawDataset::new(&self, PathBuf::from(path), None),
        }
        .apply(Ok)
    }

    /// Create a volume at path
    pub fn create_volume(
        &self,
        path: &Path,
        size: u64, // todo: byte-unit support
        props: &mut SetOnCreatePropertyValues<Volume>,
    ) -> Result<Volume> {
        props.add_value(properties::native::Volsize, size)?;
        self.create_dataset(path, zfs_cmd::dmu_objset_type::DMU_OST_ZVOL, &props.0)?;
        datasets::Volume {
            raw: RawDataset::new(&self, PathBuf::from(path), None),
        }
        .apply(Ok)
    }

    pub fn create_snapshots(
        &self,
        list: &SnapshotList,
        properties: &SetOnCreatePropertyValues<Snapshot>,
    ) -> Result<Vec<Result<Snapshot>>> {
        const SNAPS: &CStr = ::cstr::cstr!("snaps");
        const PROPS: &CStr = ::cstr::cstr!("props");
        let mut nvlist = NvList::default();
        unsafe {
            list.nvlist.add_to_nvlist(&mut nvlist, SNAPS)?;
        }
        unsafe {
            properties.0.add_to_nvlist(&mut nvlist, PROPS)?;
        }
        let mut cmd = ZfsCmd::default();
        cmd.set_name(Path::new(list.pool)).set_src(&nvlist);
        match self.run_cmd(&mut cmd, ioctl_fn::snapshot) {
            Ok(_) => Ok(list
                .paths
                .iter()
                .map(|p| {
                    datasets::Snapshot {
                        raw: RawDataset::new(&self, PathBuf::from(p), None),
                    }
                    .apply(Ok)
                })
                .collect::<Vec<_>>()),
            Err(e) => {
                let failed = match cmd.dst() {
                    Some(v) => v,
                    None => return Err(e),
                };
                list.paths
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        if let Ok(err) = failed.lookup::<i32>(&list.path_cstrs[i]) {
                            Err(Error::NixError(nix::Error::from_errno(
                                nix::errno::from_i32(err),
                            )))
                        } else {
                            datasets::Snapshot {
                                raw: RawDataset::new(&self, PathBuf::from(p), None),
                            }
                            .apply(Ok)
                        }
                    })
                    .collect::<Vec<_>>()
                    .apply(Ok)
            }
        }
    }

    pub fn create_bookmarks(&self, list: &BookmarkList) -> Result<Vec<Result<Bookmark>>> {
        let mut cmd = ZfsCmd::default();
        cmd.set_name(Path::new(list.pool)).set_src(&list.nvlist);
        match self.run_cmd(&mut cmd, ioctl_fn::bookmark) {
            Ok(_) => list
                .paths
                .iter()
                .map(|(_, q)| {
                    datasets::Bookmark {
                        raw: RawDataset::new(&self, PathBuf::from(q), None),
                    }
                    .apply(Ok)
                })
                .collect::<Vec<_>>()
                .apply(Ok),
            Err(e) => {
                let failed = match cmd.dst() {
                    Some(v) => v,
                    None => return Err(e),
                };
                list.paths
                    .iter()
                    .enumerate()
                    .map(|(i, (_, q))| {
                        if let Ok(err) = failed.lookup::<i32>(&list.path_cstrs[i].1) {
                            Err(Error::NixError(nix::Error::from_errno(
                                nix::errno::from_i32(err),
                            )))
                        } else {
                            datasets::Bookmark {
                                raw: RawDataset::new(&self, PathBuf::from(q), None),
                            }
                            .apply(Ok)
                        }
                    })
                    .collect::<Vec<_>>()
                    .apply(Ok)
            }
        }
    }

    pub fn destroy_dataset(&self, path: &Path) -> Result<()> {
        self.run_cmd(&mut ZfsCmd::default().set_name(path), ioctl_fn::destroy)?;
        Ok(())
    }

    pub(crate) fn set_properties(&self, path: &Path, properties: &NvList) -> Result<()> {
        self.run_cmd(
            &mut ZfsCmd::default().set_name(path).set_src(properties),
            ioctl_fn::set_prop,
        )?;
        Ok(())
    }

    pub fn add_hold(&self, _path: &Path, _hold: &str) {
        todo!()
    }

    pub fn get_holds(&self, _path: &Path) {
        todo!()
    }
}
