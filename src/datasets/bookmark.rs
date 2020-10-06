use crate::{datasets::*, *};
use std::{borrow::Cow, ffi::OsStr, ffi::OsString, path::Path};

fn validate_bookmark_name(name: &OsStr) -> Cow<OsStr> {
    let s = name.to_string_lossy();
    if s.starts_with('#') {
        name.into()
    } else {
        let mut n = OsString::new();
        n.push("#");
        n.push(name);
        n.into()
    }
}

pub trait BookmarkableDataset: Dataset {
    fn bookmark_path(&self, name: &OsStr) -> PathBuf {
        let mut snap_name: OsString = self.head().into();
        snap_name.push(validate_bookmark_name(name));
        PathBuf::from(snap_name)
    }
    fn bookmark(&self, name: &OsStr) -> Result<Bookmark> {
        self.handle()
            .create_bookmarks(&self.new_bookmark_list(name)?)?
            .pop()
            .expect("no bookmark value after successful call to bookmark")
    }
    fn new_bookmark_list<'a>(&'a self, name: &OsStr) -> Result<BookmarkList<'a>> {
        let mut list = BookmarkList {
            hdl: self.handle(),
            pool: self.pool(),
            paths: Vec::new(),
            path_cstrs: Vec::new(),
            nvlist: NvList::default(),
        };
        self.add_bookmark_to_list(&mut list, name)?;
        Ok(list)
    }
    fn add_bookmark_to_list<'a>(&'a self, list: &mut BookmarkList<'a>, name: &OsStr) -> Result<()> {
        list.paths.push((self.bookmark_path(name), self.path()));
        list.path_cstrs.push({
            let (bookmark_path, path) = list
                .paths
                .last()
                .expect("failed to get just added bookmark path");
            (
                CString::new(bookmark_path.as_os_str().as_bytes()).map_err(Error::from)?,
                CString::new(path.as_os_str().as_bytes()).map_err(Error::from)?,
            )
        });
        dbg!(&list);
        unsafe {
            let (bookmark_path, path) = list
                .path_cstrs
                .last()
                .expect("failed to get just added bookmark cstring");
            dbg!(bookmark_path);
            dbg!(path);
            path
                .as_ref()
                .add_to_nvlist(&mut list.nvlist, bookmark_path.as_ref())
        }
    }
}

impl BookmarkableDataset for Snapshot<'_> {}
impl BookmarkableDataset for Bookmark<'_> {}

#[derive(Debug)]
pub struct BookmarkList<'a> {
    pub(crate) hdl: &'a Handle,
    pub(crate) pool: &'a OsStr,
    pub(crate) paths: Vec<(PathBuf, &'a Path)>,
    pub(crate) path_cstrs: Vec<(CString, CString)>,
    pub(crate) nvlist: NvList,
}

impl<'a> BookmarkList<'a> {
    pub fn add<DS: BookmarkableDataset>(&mut self, ds: &'a DS, name: &OsStr) -> Result<&mut Self> {
        ds.add_bookmark_to_list(self, name)?;
        Ok(self)
    }

    pub fn bookmark(&self) -> Result<Vec<Result<Bookmark>>> {
        self.hdl.create_bookmarks(self)
    }
}
