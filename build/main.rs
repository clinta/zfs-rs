use std::{env, path::PathBuf};

#[cfg(feature = "regen")]
mod regen {
    pub mod datasets;
    pub mod native_properties;
    pub mod property_sources;
    pub mod rustfmt;
    pub mod zfs_cmd;
    pub mod zfs_ioc;
    pub use std::{fs::File, io::Write};
}

#[cfg(feature = "regen")]
use regen::*;

fn main() {
    let repo_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/_generated");
    #[cfg(not(feature = "regen"))]
    {
        println!("cargo:rustc-env=ZFSRS_INCLUDE_DIR={}", repo_path.display());
    }

    #[cfg(feature = "regen")]
    {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        println!("cargo:rustc-env=ZFSRS_INCLUDE_DIR={}", out_path.display());
        zfs_cmd::zfs_cmd_bindings(&out_path.join("zfs_cmd.rs"));
        rustfmt::write_formatted(&out_path.join("zfs_ioc.rs"), &zfs_ioc::gen_zfs_ioctls());
        rustfmt::write_formatted(
            &out_path.join("native_properties.rs"),
            &native_properties::gen_native_properties(),
        );
        rustfmt::write_formatted(
            &out_path.join("property_sources.rs"),
            &property_sources::gen_property_sources(),
        );
        rustfmt::write_formatted(&out_path.join("datasets.rs"), &datasets::gen_zfs_datasets());

        println!(
            "cargo:warning=bindings written. `cp {}/* {}` to update the repository copy",
            out_path.display(),
            &repo_path.display()
        );
    }
}
