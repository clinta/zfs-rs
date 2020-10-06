use proc_macro_crate::crate_name;
use quote::{format_ident, quote};
use strum::IntoEnumIterator;

pub fn gen_zfs_ioctls() -> String {
    let nix = crate_name("nix").expect("unable to find crate name for nix");
    let ioctl_readwrite_bad: syn::Path = syn::parse_str(&format!("::{}::ioctl_readwrite_bad", nix))
        .expect("unable to parse path to ioctl_readwrite_bad");
    let zfs_cmd: syn::Path = syn::parse_str("crate::zfs_cmd::zfs_cmd_bindings::zfs_cmd")
        .expect("unable to parse path to zfs_cmd");

    let (name, ioctl): (Vec<_>, Vec<_>) = zfs_sys::zfs_ioc::iter()
        .map(|ioctl| {
            (
                format_ident!(
                    "{}",
                    <&'static str>::from(ioctl)
                        .strip_prefix("ZFS_IOC_")
                        .unwrap()
                        .to_lowercase()
                ),
                ioctl as u32,
            )
        })
        .unzip();

    let out = quote! {
        #(
            #ioctl_readwrite_bad!(#name, #ioctl, #zfs_cmd);
        )*
    };

    out.to_string()
}
