use crate::*;

pub struct ChildIterator<'a> {
    hdl: &'a Handle,
    path: &'a Path,
    zcmd: zfs_cmd::ZfsCmd,
}

impl<'a> ChildIterator<'a> {
    pub fn new(hdl: &'a Handle, path: &'a Path) -> Self {
        Self {
            hdl,
            path,
            zcmd: zfs_cmd::ZfsCmd::default(),
        }
    }
}

fn run_next<'a>(
    hdl: &'a Handle,
    path: &Path,
    zcmd: &mut zfs_cmd::ZfsCmd,
) -> Option<Result<Datasets<'a>>> {
    zcmd.set_name(path);
    match hdl.run_cmd(zcmd, ioctl_fn::dataset_list_next) {
        Ok(_) => Some(Datasets::try_from(RawDataset::new(
            hdl,
            path.into(),
            zcmd.dst(),
        ))),
        Err(Error::NixError(nix::Error::Sys(nix::errno::Errno::ESRCH))) => None,
        Err(e) => Some(Err(e)),
    }
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = Result<Datasets<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        run_next(self.hdl, self.path, &mut self.zcmd)
    }
}

pub struct ChildIteratorRecursive<'a> {
    hdl: &'a Handle,
    children: Vec<(PathBuf, zfs_cmd::ZfsCmd)>,
}

impl<'a> ChildIteratorRecursive<'a> {
    pub fn new(hdl: &'a Handle, path: PathBuf) -> Self {
        Self {
            hdl,
            children: vec![(path, zfs_cmd::ZfsCmd::default())],
        }
    }
}

impl<'a> Iterator for ChildIteratorRecursive<'a> {
    type Item = Result<Datasets<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((path, zcmd)) = self.children.last_mut() {
            let path: &Path = path;
            match run_next(self.hdl, path, zcmd) {
                None => {
                    self.children.pop();
                    self.next()
                }
                Some(Err(e)) => Some(Err(e)),
                Some(Ok(Datasets::Filesystem(child))) => {
                    self.children
                        .push((child.path().into(), zfs_cmd::ZfsCmd::default()));
                    Some(Ok(Datasets::Filesystem(child)))
                }
                Some(Ok(child)) => Some(Ok(child)),
            }
        } else {
            None
        }
    }
}
