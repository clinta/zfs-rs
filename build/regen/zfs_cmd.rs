extern crate bindgen;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn zfs_cmd_bindings(out_file: &Path) {
    let zfs_src_dir =
        PathBuf::from(env::var("ZFS_SRC").unwrap_or_else(|_| "/usr/src/zfs/".to_string()));

    let bindings = bindgen::Builder::default()
        .header_contents(
            "bindings.h",
            "
#include <sys/zfs_ioctl.h>
#include <sys/types.h>
",
        )
        .clang_args(vec![
            format!("-I{}", zfs_src_dir.join("include").display()),
            format!("-I{}", zfs_src_dir.join("lib/libspl/include").display()),
            format!(
                "-I{}",
                zfs_src_dir.join("lib/libspl/include/os/linux").display()
            ),
        ])
        .whitelist_type("zfs_cmd")
        //.whitelist_type("zfs_ioc")
        .rustified_enum("boolean_t")
        .rustified_enum("dmu_objset_type")
        .derive_default(true)
        .derive_debug(true)
        .generate()
        .expect("Unable to generate zfs_cmd bindings")
        .to_string();

    fs::write(out_file, bindings).expect("couldn't write zfs_cmd bindings");
}
