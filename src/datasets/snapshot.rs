use crate::{datasets::*, *};
use std::{borrow::Cow, ffi::OsStr, ffi::OsString};

fn validate_snapshot_name(name: &OsStr) -> Cow<OsStr> {
    let s = name.to_string_lossy();
    if s.starts_with('@') {
        name.into()
    } else {
        let mut n = OsString::new();
        n.push("@");
        n.push(name);
        n.into()
    }
}

pub trait SnapshotableDataset: Dataset {
    fn snapshot_path(&self, name: &OsStr) -> PathBuf {
        let mut snap_name = self.name().to_os_string();
        snap_name.push(validate_snapshot_name(name));
        self.path().with_file_name(snap_name)
    }
    fn snapshot(
        &self,
        name: &OsStr,
        properties: &SetOnCreatePropertyValues<Snapshot>,
    ) -> Result<Snapshot> {
        self.handle()
            .create_snapshots(&self.new_snapshot_list(name)?, properties)?
            .pop()
            .expect("no snapshot value after successful call to snapshot")
    }
    fn new_snapshot_list<'a>(&'a self, name: &OsStr) -> Result<SnapshotList<'a>> {
        let mut list = SnapshotList {
            hdl: self.handle(),
            pool: self.pool(),
            paths: Vec::new(),
            path_cstrs: Vec::new(),
            nvlist: NvList::default(),
        };
        self.add_snapshot_to_list(&mut list, name)?;
        Ok(list)
    }
    fn add_snapshot_to_list(&self, list: &mut SnapshotList, name: &OsStr) -> Result<()> {
        list.paths.push(self.snapshot_path(name));
        list.path_cstrs.push(
            CString::new(
                list.paths
                    .last()
                    .expect("failed to get just added path")
                    .as_os_str()
                    .as_bytes(),
            )
            .map_err(Error::from)?,
        );
        unsafe {
            true.add_to_nvlist(
                &mut list.nvlist,
                list.path_cstrs
                    .last()
                    .expect("failed to get just added snapshot cstring"),
            )
        }
    }
}

impl SnapshotableDataset for Filesystem<'_> {}
impl SnapshotableDataset for Volume<'_> {}

#[derive(Debug)]
pub struct SnapshotList<'a> {
    pub(crate) hdl: &'a Handle,
    pub(crate) pool: &'a OsStr,
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) path_cstrs: Vec<CString>,
    pub(crate) nvlist: NvList,
}

impl<'a> SnapshotList<'a> {
    pub fn add<DS: SnapshotableDataset>(&mut self, ds: &DS, name: &OsStr) -> Result<&mut Self> {
        ds.add_snapshot_to_list(self, name)?;
        Ok(self)
    }

    pub fn snapshot(
        &self,
        properties: &SetOnCreatePropertyValues<Snapshot>,
    ) -> Result<Vec<Result<Snapshot>>> {
        self.hdl.create_snapshots(self, properties)
    }
}
