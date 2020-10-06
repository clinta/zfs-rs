#![allow(dead_code)]

use crate::*;
use std::{ffi::OsStr, os::raw::c_int, os::unix::ffi::OsStrExt, path::Path};

mod zfs_cmd_bindings;
use zfs_cmd_bindings::zfs_cmd;
pub(crate) mod ioctl_fn;
pub(crate) use zfs_cmd_bindings::dmu_objset_type;

#[derive(Default)]
pub struct ZfsCmd(zfs_cmd);

impl Drop for ZfsCmd {
    fn drop(&mut self) {
        let zc = self.0;
        unsafe {
            nvpair_sys::fnvlist_pack_free(zc.zc_nvlist_src as *mut _, zc.zc_nvlist_src_size);
            nvpair_sys::fnvlist_pack_free(zc.zc_nvlist_dst as *mut _, zc.zc_nvlist_dst_size);
            nvpair_sys::fnvlist_pack_free(zc.zc_nvlist_conf as *mut _, zc.zc_nvlist_conf_size);
        }
    }
}

impl ZfsCmd {
    pub fn name(&self) -> &Path {
        OsStr::from_bytes(unsafe { std::ffi::CStr::from_ptr(self.0.zc_name.as_ptr()) }.to_bytes())
            .as_ref()
    }

    pub fn set_name(&mut self, name: &Path) -> &mut Self {
        self.0.zc_name = [0; 4096];
        let path_bytes = unsafe { &*(name.as_os_str().as_bytes() as *const [u8] as *const [i8]) };
        self.0.zc_name[..path_bytes.len()].clone_from_slice(path_bytes);
        self
    }

    pub fn set_src(&mut self, source: &NvList) -> &mut Self {
        let mut size = 0;
        self.0.zc_nvlist_src =
            unsafe { nvpair_sys::fnvlist_pack(source.nvlist(), &mut size) } as u64;
        self.0.zc_nvlist_src_size = size;
        self
    }

    pub fn dst(&mut self) -> Option<NvList> {
        if self.0.zc_nvlist_dst_filled.into() {
            unsafe {
                nvpair_sys::fnvlist_unpack(
                    self.0.zc_nvlist_dst as *mut _,
                    self.0.zc_nvlist_dst_size,
                )
                .as_mut()
            }
            .map(NvList::from)
        } else {
            None
        }
    }

    pub fn cookie(&self) -> u64 {
        self.0.zc_cookie
    }

    pub fn set_cookie(&mut self, cookie: u64) -> &mut Self {
        self.0.zc_cookie = cookie;
        self
    }

    pub fn run(&mut self, fd: c_int, request: RequestFn) -> Result<&mut Self> {
        let zc = &mut self.0;
        if zc.zc_nvlist_dst != 0 {
            unsafe {
                nvpair_sys::fnvlist_pack_free(zc.zc_nvlist_dst as *mut _, zc.zc_nvlist_dst_size)
            };
        }
        zc.zc_nvlist_dst_size = std::cmp::max(zc.zc_nvlist_src_size, 128 * 1024);
        zc.zc_nvlist_dst = unsafe { libc::malloc(zc.zc_nvlist_dst_size as usize) } as u64;
        if zc.zc_nvlist_dst == 0 {
            panic!("failed to allocate memory")
        }

        unsafe { request(fd, zc) }?;
        Ok(self)
    }
}

pub(crate) type RequestFn = unsafe fn(fd: c_int, data: *mut zfs_cmd) -> nix::Result<c_int>;
