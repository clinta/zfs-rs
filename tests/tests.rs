#![allow(dead_code)]

use ::std::thread::sleep;
use anyhow::{anyhow, Result};
use datasets::SnapshotableDataset;
use once_cell::sync::Lazy; // 1.4.0
use std::{
    ffi::OsStr, io::stdin, path::Path, path::PathBuf, process::Command, sync::LockResult,
    sync::RwLockReadGuard, sync::RwLockWriteGuard,
};
use std::{sync::RwLock, time::Duration};
use zfs_rs::{properties::*, *};

static THE_RESOURCE: Lazy<RwLock<()>> = Lazy::new(RwLock::default);

struct TestPool(String, tempfile::NamedTempFile);

impl TestPool {
    fn new() -> Result<Self> {
        let file = tempfile::NamedTempFile::new()?;
        let pool_name: String = file
            .path()
            .file_name()
            .ok_or_else(|| anyhow!("no file name"))?
            .to_string_lossy()
            .chars()
            .filter(|c| (*c).is_alphanumeric())
            .collect::<String>();
        Command::new("truncate")
            .arg("-s")
            .arg("1G")
            .arg(file.path())
            .status()?;
        Command::new("zpool")
            .arg("create")
            //.arg("-d")
            .arg(&pool_name)
            .arg(file.path())
            .status()?;
        Ok(Self(pool_name, file))
    }

    fn name(&self) -> &Path {
        &Path::new(&self.0)
    }
}

impl Drop for TestPool {
    fn drop(&mut self) {
        Command::new("zpool")
            .arg("destroy")
            .arg(&self.0)
            .status()
            .expect("failed to destroy pool");
    }
}

fn init<'a>() -> Result<(LockResult<RwLockReadGuard<'a, ()>>, Handle, TestPool)> {
    Ok((THE_RESOURCE.read(), Handle::new()?, TestPool::new()?))
}

fn init_exclusive<'a>() -> Result<(LockResult<RwLockWriteGuard<'a, ()>>, Handle, TestPool)> {
    Ok((THE_RESOURCE.write(), Handle::new()?, TestPool::new()?))
}

fn _wait() {
    println!("press enter to continue");
    stdin().read_line(&mut String::new()).unwrap();
}

#[test]
fn get_dataset() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    hdl.get_filesystem(pool.name())?;
    Ok(())
}

/*
#[test]
#[ignore]
fn get_dataset_zpm() -> Result<()> {
    let hdl = Handle::new()?;
    let ds = hdl.get_filesystem(&PathBuf::from("zpmanjaro"))?;
    println!("{}", ds.property::<properties::types::Compression>()?);
    println!("{}", ds.property::<properties::types::Creation>()?);
    println!("{}", ds.property::<properties::types::Casesensitivity>()?);
    Ok(())
}
*/

#[test]
fn test_pool_configs() -> Result<()> {
    let (_l, hdl, pool1) = init_exclusive()?;
    let pool2 = TestPool::new()?;
    let root_fs = hdl
        .root_filesystems()?
        .collect::<zfs_rs::Result<Vec<_>>>()?;
    let mut root_paths = root_fs.iter().map(|x| x.path()).collect::<Vec<_>>();
    let mut expected_paths = vec![pool1.name(), pool2.name()];
    root_paths.sort();
    expected_paths.sort();
    assert_eq!(root_paths, expected_paths);
    Ok(())
}

#[test]
fn set_user_property() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let mut ds = hdl.get_filesystem(pool.name())?;
    let prop = UserProperty::new("foo:foo")?;
    ds.set_property(&prop, "bar")?;
    let prop_v = ds.property(&prop)?.unwrap();
    assert_eq!("bar", prop_v.value());
    assert_eq!(Source::Local, prop_v.source());
    Ok(())
}

#[test]
fn create_filesystem() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    ds.create_filesystem(&PathBuf::from("foo"), &SetOnCreatePropertyValues::default())?;
    Ok(())
}

#[test]
fn create_volume() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    let foo = ds.create_volume(
        &PathBuf::from("foo"),
        2048 << 16,
        &mut SetOnCreatePropertyValues::default(),
    )?;
    let volsize = foo.property(&native::Volsize)?.unwrap().value();
    assert_eq!(volsize, 2048 << 16);
    Ok(())
}

#[test]
fn destroy_filesystem() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    let foo = ds.create_filesystem(&PathBuf::from("foo"), &SetOnCreatePropertyValues::default())?;
    foo.destroy()?;
    match hdl.get_filesystem(&pool.name().join("foo")) {
        Err(Error::NixError(nix::Error::Sys(nix::errno::Errno::ENOENT))) => {}
        _ => panic!("foo should not be able to be gotten after being destroyed"),
    }
    Ok(())
}

#[test]
fn destroy_volume() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    let foo = ds.create_volume(
        &PathBuf::from("foo"),
        2048 << 16,
        &mut SetOnCreatePropertyValues::default(),
    )?;
    sleep(Duration::from_millis(500)); // sleep required or EBUSY is returned
    foo.destroy()?;
    match hdl.get_filesystem(&pool.name().join("foo")) {
        Err(Error::NixError(nix::Error::Sys(nix::errno::Errno::ENOENT))) => {}
        _ => panic!("foo should not be able to be gotten after being destroyed"),
    }
    Ok(())
}

#[test]
fn create_filesystem_snapshot() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    ds.snapshot(&OsStr::new("foo"), &SetOnCreatePropertyValues::default())?;
    Ok(())
}

#[test]
fn destroy_snapshot() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    let fs1 = ds.create_filesystem(&PathBuf::from("fs1"), &SetOnCreatePropertyValues::default())?;
    let sn = fs1.snapshot(&OsStr::new("foo"), &SetOnCreatePropertyValues::default())?;
    sn.destroy()?;
    match hdl.get_filesystem(&pool.name().join("fs1@foo")) {
        Err(Error::NixError(nix::Error::Sys(nix::errno::Errno::ENOENT))) => {}
        v => {
            dbg!(&v);
            panic!("foo snapshot should not exist after being destroyed")
        }
    }
    Ok(())
}

#[test]
fn create_snapshots() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    let fs1 = ds.create_filesystem(&PathBuf::from("fs1"), &SetOnCreatePropertyValues::default())?;
    let fs2 = ds.create_filesystem(&PathBuf::from("fs2"), &SetOnCreatePropertyValues::default())?;
    let _fs3 =
        ds.create_filesystem(&PathBuf::from("fs3"), &SetOnCreatePropertyValues::default())?;
    let v1 = ds.create_volume(
        &PathBuf::from("v1"),
        2048 << 16,
        &mut SetOnCreatePropertyValues::default(),
    )?;
    let v2 = ds.create_volume(
        &PathBuf::from("v2"),
        2048 << 16,
        &mut SetOnCreatePropertyValues::default(),
    )?;
    let _v3 = ds.create_volume(
        &PathBuf::from("v3"),
        2048 << 16,
        &mut SetOnCreatePropertyValues::default(),
    )?;
    //let snap_ds: Vec<SnapshotableDatasets> = vec![fs1.into(), fs2.into(), v1.into(), v2.into()];
    //let snaps = snap_ds.snapshot(&OsStr::new("foo"), &SetOnCreatePropertyValues::default())?;
    let snap_name = OsStr::new("foo");
    let mut snaps = fs1.new_snapshot_list(snap_name)?;
    let snaps = snaps
        .add(&fs2, snap_name)?
        .add(&v1, snap_name)?
        .add(&v2, snap_name)?
        .snapshot(&SetOnCreatePropertyValues::default())?;
    let snaps: Vec<Snapshot> = snaps.into_iter().collect::<zfs_rs::Result<Vec<_>>>()?;
    let mut iter = snaps.iter();
    assert_eq!(
        iter.next().unwrap(),
        &hdl.get_snapshot(&pool.name().join("fs1@foo"))?
    );
    assert_eq!(
        iter.next().unwrap(),
        &hdl.get_snapshot(&pool.name().join("fs2@foo"))?
    );
    assert_eq!(
        iter.next().unwrap(),
        &hdl.get_snapshot(&pool.name().join("v1@foo"))?
    );
    assert_eq!(
        iter.next().unwrap(),
        &hdl.get_snapshot(&pool.name().join("v2@foo"))?
    );
    let txg = snaps
        .iter()
        .next()
        .unwrap()
        .property(&native::Createtxg)
        .unwrap()
        .unwrap()
        .value();
    assert!(snaps
        .iter()
        .all(|s| s.property(&native::Createtxg).unwrap().unwrap().value() == txg));
    match hdl.get_snapshot(&pool.name().join("fs3@foo")) {
        Err(Error::NixError(nix::Error::Sys(nix::errno::Errno::ENOENT))) => {}
        _ => panic!("fs3 should not have a snapshot"),
    }
    match hdl.get_snapshot(&pool.name().join("v3@foo")) {
        Err(Error::NixError(nix::Error::Sys(nix::errno::Errno::ENOENT))) => {}
        _ => panic!("v3 should not have a snapshot"),
    }
    Ok(())
}

#[test]
fn create_bookmarks() -> Result<()> {
    dbg!("here");
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    let fs1 = ds.create_filesystem(&PathBuf::from("fs1"), &SetOnCreatePropertyValues::default())?;
    let fs2 = ds.create_filesystem(&PathBuf::from("fs2"), &SetOnCreatePropertyValues::default())?;
    let v1 = ds.create_volume(
        &PathBuf::from("v1"),
        2048 << 16,
        &mut SetOnCreatePropertyValues::default(),
    )?;
    let v2 = ds.create_volume(
        &PathBuf::from("v2"),
        2048 << 16,
        &mut SetOnCreatePropertyValues::default(),
    )?;
    let snap_name = OsStr::new("snap1");
    let mut snaps = fs1.new_snapshot_list(snap_name)?;
    let snaps = snaps
        .add(&fs2, snap_name)?
        .add(&v1, snap_name)?
        .add(&v2, snap_name)?
        .snapshot(&SetOnCreatePropertyValues::default())?
        .into_iter()
        .collect::<zfs_rs::Result<Vec<Snapshot>>>()?;
    dbg!("here");

    let bookmark_name = &OsStr::new("bookmark1");
    let mut bookmarks_list = snaps[0].new_bookmark_list(&bookmark_name)?;
    dbg!("here");
    for snap in snaps.iter().skip(1) {
        bookmarks_list.add(snap, &bookmark_name)?;
    }
    dbg!("here");
    _wait();
    bookmarks_list.bookmark()?;
    dbg!("here");

    _wait();
    let fs1_bookmark = hdl.get_bookmark(&pool.name().join("fs1#bookmark1"))?;
    dbg!("here");
    hdl.get_bookmark(&pool.name().join("fs2#bookmark1"))?;
    dbg!("here");
    hdl.get_bookmark(&pool.name().join("v1#bookmark1"))?;
    dbg!("here");
    hdl.get_bookmark(&pool.name().join("v2#bookmark1"))?;

    dbg!("here");
    let fs1_bookmark2 = fs1_bookmark.bookmark(&OsStr::new("bookmark2"))?;

    dbg!("here");
    assert_eq!(
        fs1_bookmark.property(&native::Createtxg)?.unwrap().value(),
        fs1_bookmark2.property(&native::Createtxg)?.unwrap().value()
    );
    dbg!("here");
    Ok(())
}

/*
#[test]
fn get_all_filesystem_properties() -> Result<()> {
    let (_l, hdl, pool) = init()?;
    let ds = hdl.get_filesystem(pool.name())?;
    for p in properties::FilesystemProperties::iter_types() {
        println!(
            "{:#?}",
            p.from_dataset(&ds).or_default().into_option().result()?
        );
    }
    Ok(())
}
*/

/*
#[test]
fn test_dataset_list_next() {
    let hdl = Handle::new().unwrap();
    let mut zcmd = ZfsCmd::default();
    zcmd.set_name(&PathBuf::from("zpmanjaro/test"));

    while let Ok(_) = hdl.run_cmd(&mut zcmd, ioctl_fn::dataset_list_next) {
        println!("name: {}", zcmd.name().display());
        zcmd.dst().unwrap()._dump();
        zcmd.set_name(&PathBuf::from("zpmanjaro/test"));
    }
}

#[test]
fn test_get_filesystem() {
    Handle::new()
        .unwrap()
        .get_filesystem(&PathBuf::from("zpmanjaro/test"))
        .unwrap();
}

#[test]
fn test_get_filesystem_on_volume() {
    Handle::new()
        .unwrap()
        .get_filesystem(&PathBuf::from("zpmanjaro/test/v"))
        .expect_err("get filesytem on volume should fail");
}

#[test]
fn test_get_volume() {
    Handle::new()
        .unwrap()
        .get_volume(&PathBuf::from("zpmanjaro/test/v"))
        .unwrap();
}

#[test]
fn test_get_volume_on_filesystem() {
    Handle::new()
        .unwrap()
        .get_volume(&PathBuf::from("zpmanjaro/test/foo"))
        .expect_err("get filesytem on volume should fail");
}

#[test]
fn test_root_filesystems() {
    for ds in Handle::new().unwrap().root_filesystems().unwrap() {
        println!("root name: {}", ds.unwrap().path().display())
    }
}

*/
